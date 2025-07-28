mod calculate;
mod parse;

#[derive(Clone, Debug)]
pub enum Die {
    Fate,
    Number(i64),
}

#[derive(Clone, Debug)]
pub enum Keep {
    All,
    Highest(u64),
    Lowest(u64),
}

#[derive(Clone, Debug)]
pub struct Dice {
    pub count: u64,
    pub die: Die,
    pub keep: Keep,
}

#[derive(Clone, Debug)]
pub enum Category {
    Dice(Dice),
    Number(i64),
    OpPlus,
    OpMinus,
    Space,
    None,
}

#[derive(Clone, Debug)]
pub struct Entry {
    original: String,
    category: Category,
}

#[derive(Debug, Default)]
pub struct Value {
    pub final_value: Option<i64>,
    pub sub_values: Vec<(i64, bool)>,
}

#[derive(Debug, Default)]
pub struct Formula {
    pub original: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Default)]
pub struct Result {
    pub final_value: i64,
    pub grouped_text: String,
    pub partial_text: String,
    pub formula_text: String,
}

#[derive(Debug)]
pub enum Issue {
    MalformedEntries,
    InvalidOperator,
    InvalidNumber,
    InvalidDice,
    Undefined,
}

#[derive(Debug)]
pub struct FormulaError {
    pub original: String,
    pub issue: Issue,
    pub issue_ix: Option<usize>,
}

impl Entry {
    fn dice(original: String, count: u64, die: Die, keep: Keep) -> Self {
        Entry {
            original,
            category: Category::Dice(Dice { count, die, keep }),
        }
    }

    fn number(original: String, value: u64) -> Self {
        let v = value.try_into().unwrap_or(i64::MAX);

        Entry {
            original,
            category: Category::Number(v),
        }
    }

    fn op_plus(original: String) -> Self {
        Entry {
            original,
            category: Category::OpPlus,
        }
    }

    fn op_minus(original: String) -> Self {
        Entry {
            original,
            category: Category::OpMinus,
        }
    }

    fn space(original: String) -> Self {
        Entry {
            original,
            category: Category::Space,
        }
    }
}

impl Formula {
    pub fn generate_result(&self) -> Result {
        let mut values = vec![];

        // generate values for entries
        for entry in &self.entries {
            let value = calculate::process_entry(entry);
            values.push(value);
        }

        // setup for iteration
        let mut operator = Category::OpPlus;
        let mut final_value = 0;
        let mut grouped_text = String::new();
        let mut partial_text = String::new();
        let mut formula_text = String::new();

        // iterate over entries
        for (ix, entry) in self.entries.iter().enumerate() {
            let f_value = values[ix].final_value.unwrap_or(0);
            let s_values = &values[ix].sub_values;

            // calculate final result
            match &entry.category {
                Category::Number(_) | Category::Dice(_) => match operator {
                    Category::OpPlus => final_value += f_value,
                    Category::OpMinus => final_value -= f_value,
                    _ => {}
                },
                Category::OpPlus => operator = Category::OpPlus,
                Category::OpMinus => operator = Category::OpMinus,
                _ => {}
            }

            // assemble grouped text from final values
            if !grouped_text.is_empty() {
                grouped_text.push(' ');
            }
            match &entry.category {
                Category::Number(_) => grouped_text.push_str(&f_value.to_string()),
                Category::Dice(_) => {
                    let block = format!("[ {f_value} ]");
                    grouped_text.push_str(&block);
                }
                _ => grouped_text.push_str(&entry.original),
            }

            // assemble partial text from sub values
            if !partial_text.is_empty() {
                partial_text.push(' ');
            }
            match &entry.category {
                Category::Number(_) => partial_text.push_str(&f_value.to_string()),
                Category::Dice(_) => {
                    // inactive values must be stricken out with '~'
                    let list = s_values
                        .iter()
                        .map(|(value, active)| {
                            if *active {
                                value.to_string()
                            } else {
                                format!("~~{value}~~")
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(", ");
                    let block = format!("[ {list} ]");

                    partial_text.push_str(&block);
                }
                _ => partial_text.push_str(&entry.original),
            }

            // assemble formula text from original snippets
            if !formula_text.is_empty() {
                formula_text.push(' ');
            }
            formula_text.push_str(&entry.original);
        }

        Result {
            final_value,
            grouped_text,
            partial_text,
            formula_text,
        }
    }
}
