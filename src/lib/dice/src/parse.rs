use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{map, opt, value},
    error::make_error,
    multi::many0,
};

fn get_delta(original: &str, remaining: &str) -> String {
    // calculate delta of length
    let original_len = original.len();
    let remaining_len = remaining.len();
    let delta = original_len - remaining_len;

    // create delta string
    original[..delta].to_string()
}

fn get_dice(i: &str) -> IResult<&str, crate::Entry> {
    // optional count
    let (input, result) = opt(nom::character::complete::u64).parse(i)?;
    let count = result.unwrap_or(1);

    // required d or D for DICE
    let (input, _) = alt((tag("d"), tag("D"))).parse(input)?;

    // required dice face F or a positive number
    let (input, die) = alt((
        map(nom::character::complete::u64, |u| {
            crate::Die::Number(u.try_into().unwrap_or(i64::MAX))
        }),
        value(crate::Die::Fate, tag("F")),
    ))
    .parse(input)?;

    // optional keep syntax
    let (input, s) = opt(alt((tag(":H"), tag(":L"), tag(":h"), tag(":l")))).parse(input)?;
    if let Some(k) = s {
        // set basic type and value
        let mut keep = match k {
            ":H" | ":h" => crate::Keep::Highest(1),
            ":L" | ":l" => crate::Keep::Lowest(1),
            _ => crate::Keep::All,
        };

        // optional number here
        let (input, s) = opt(nom::character::complete::u64).parse(input)?;
        if let Some(number) = s {
            if count <= number {
                keep = crate::Keep::All;
            }

            match keep {
                crate::Keep::Highest(_) => keep = crate::Keep::Highest(number),
                crate::Keep::Lowest(_) => keep = crate::Keep::Lowest(number),
                crate::Keep::All => {}
            }
        }

        let original = get_delta(i, input);
        Ok((input, crate::Entry::dice(original, count, die, keep)))
    } else {
        let original = get_delta(i, input);
        Ok((
            input,
            crate::Entry::dice(original, count, die, crate::Keep::All),
        ))
    }
}

fn get_operator(i: &str) -> IResult<&str, crate::Entry> {
    let (input, category) = alt((
        value(crate::Category::OpPlus, tag("+")),
        value(crate::Category::OpMinus, tag("-")),
    ))
    .parse(i)?;
    let original = get_delta(i, input);

    match category {
        crate::Category::OpPlus => Ok((input, crate::Entry::op_plus(original))),
        crate::Category::OpMinus => Ok((input, crate::Entry::op_minus(original))),
        _ => unreachable!(), // we only match on OpPlus and OpMinus
    }
}

fn get_number(i: &str) -> IResult<&str, crate::Entry> {
    let (input, number) = nom::character::complete::u64(i)?;
    let original = get_delta(i, input);

    Ok((input, crate::Entry::number(original, number)))
}

fn get_space(i: &str) -> IResult<&str, crate::Entry> {
    let (input, _) = space1(i)?;
    let original = get_delta(i, input);

    Ok((input, crate::Entry::space(original)))
}
fn get_noise(i: &str) -> IResult<&str, crate::Entry> {
    Err(nom::Err::Error(make_error(i, nom::error::ErrorKind::Char)))
}

pub fn get_formula(i: &str) -> IResult<&str, crate::Formula> {
    let (input, entries) = many0(alt((
        get_dice,
        get_operator,
        get_number,
        get_space,
        get_noise,
    )))
    .parse(i)?;

    if !input.is_empty() {
        // there is a remainder, so parsing was not complete
        return Err(nom::Err::Error(make_error(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    Ok((
        input,
        crate::Formula {
            original: i.into(),
            entries,
        },
    ))
}

impl crate::Formula {
    fn into_simplified(self) -> Result<Self, crate::FormulaError> {
        // temporary storage
        let mut entries = vec![];

        // setup for iteration
        let mut was_operator: Option<bool> = None;
        let mut current_ix = 0;

        // iterate over original entries
        for entry in self.entries {
            match entry.category {
                crate::Category::Dice(ref dice) => {
                    // disassemble dice
                    let count = dice.count;
                    let faces = match dice.die {
                        crate::Die::Number(faces) => faces,
                        crate::Die::Fate => 3,
                    };

                    // check if previous entry was an operator
                    if let Some(was_op) = was_operator {
                        if !was_op {
                            return Err(crate::FormulaError {
                                original: self.original,
                                issue: crate::Issue::InvalidDice,
                                issue_ix: Some(current_ix),
                            });
                        }
                    }

                    // check dice against constraints
                    if count > 100 || faces > 1000 {
                        return Err(crate::FormulaError {
                            original: self.original,
                            issue: crate::Issue::InvalidDice,
                            issue_ix: Some(current_ix),
                        });
                    }

                    // shift ix & flag
                    was_operator = Some(false);
                    current_ix += entry.original.len();

                    // add current dice to entries
                    entries.push(entry);
                }
                crate::Category::Number(_) => {
                    // check if previous entry was an operator
                    if let Some(was_op) = was_operator {
                        if !was_op {
                            return Err(crate::FormulaError {
                                original: self.original,
                                issue: crate::Issue::InvalidNumber,
                                issue_ix: Some(current_ix),
                            });
                        }
                    }

                    // shift ix & flag
                    current_ix += entry.original.len();
                    was_operator = Some(false);

                    // add number to entries
                    entries.push(entry);
                }
                crate::Category::OpPlus | crate::Category::OpMinus => {
                    // check if previous entry was an operator
                    if let Some(was_op) = was_operator {
                        if was_op {
                            return Err(crate::FormulaError {
                                original: self.original,
                                issue: crate::Issue::InvalidOperator,
                                issue_ix: Some(current_ix),
                            });
                        }
                    }

                    // shift ix & flag
                    current_ix += entry.original.len();
                    was_operator = Some(true);

                    // add operator to entries
                    entries.push(entry);
                }
                _ => {
                    // shift ix
                    current_ix += entry.original.len();
                }
            }
        }

        // create formula
        Ok(crate::Formula {
            original: self.original,
            entries,
        })
    }
}

impl TryFrom<&str> for crate::Formula {
    type Error = crate::FormulaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // try the conversion as is
        let result = get_formula(value);

        match result {
            Ok((_, formula)) => {
                // try to simplify the formula
                match formula.into_simplified() {
                    Ok(simplified) => Ok(simplified),
                    Err(issue) => Err(issue),
                }
            }
            Err(err) => {
                // handle the simple parsing errors here
                match err {
                    nom::Err::Error(e) => {
                        let issue_ix = Some(value.len() - e.input.len());
                        Err(crate::FormulaError {
                            original: value.to_owned(),
                            issue: crate::Issue::MalformedEntries,
                            issue_ix,
                        })
                    }
                    _ => Err(crate::FormulaError {
                        original: value.to_owned(),
                        issue: crate::Issue::Undefined,
                        issue_ix: None,
                    }),
                }
            }
        }
    }
}

impl TryFrom<&String> for crate::Formula {
    type Error = crate::FormulaError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        crate::Formula::try_from(value.as_str())
    }
}
