pub fn result_message(reason: Option<&String>, result: &dice::Result) -> (String, String) {
    let result_line = match reason {
        Some(s) => format!("{s} = **{}**", result.final_value),
        None => format!("= **{}**", result.final_value),
    };

    let formula_line = if result.grouped_text == result.partial_text {
        format!("= {} = {}", result.grouped_text, result.formula_text)
    } else {
        format!(
            "= {} = {} = {}",
            result.grouped_text, result.partial_text, result.formula_text
        )
    };

    // assemble lines as a discord message
    if formula_line.len() <= crate::FORMULA_LINE_LIMIT {
        (result_line, formula_line)
    } else {
        (result_line, format!("= ... = {}", result.formula_text))
    }
}

pub fn dice_error_message(error: &dice::FormulaError) -> String {
    let error_line = match error.issue {
        dice::Issue::MalformedEntries => "malformed entries in dice formula",
        dice::Issue::InvalidOperator => "invalid operator in dice formula",
        dice::Issue::InvalidNumber => "invalid number in dice formula",
        dice::Issue::InvalidDice => "invalid dice in dice formula",
        dice::Issue::Undefined => "undefined error in dice formula",
    };

    // generate code block with error location
    let arrow = " ".repeat(error.issue_ix.unwrap_or(0));
    format!("{}\n```\n{}\n{}^\n```", error_line, error.original, arrow)
}

pub fn coin_flip_message(heads: bool) -> (String, String) {
    let side = if heads { "HEADS" } else { "TAILS" };
    (format!("coin flip = **{side}**"), String::new())
}
