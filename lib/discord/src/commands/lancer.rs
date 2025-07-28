/// command to roll a standard Lancer attack / check / save
#[poise::command(slash_command, rename = "lancer-roll", category = "lancer")]
pub async fn lancer_roll(
    ctx: super::Context<'_>,
    #[rename = "modifier"]
    #[description = "fixed modifier from mech skill / trigger / other stat"]
    modifier: i64,
    #[rename = "accuracy-or-difficulty"]
    #[description = "the degree of total accuracy or difficulty [0: normal, +N: accuracy, -N: difficulty]"]
    sixes: i64,
    #[min_length = 1]
    #[max_length = 64]
    #[rename = "reason"]
    #[description = "short identifier for the reason of this roll"]
    reason: Option<String>,
) -> Result<(), super::Error> {
    // assemble a string formula by hand
    let dice_formula = if sixes == 0 {
        format!("d20{modifier:+}")
    } else {
        format!("d20{modifier:+}{sixes:+}d6:H")
    };

    // this formula can always be parsed
    let formula = dice::Formula::try_from(&dice_formula).unwrap_or_default();

    // generate result for formula
    let result = formula.generate_result();

    // generate message body and reply
    let msg = crate::message::result_message(reason, &result);
    ctx.reply(msg).await?;

    Ok(())
}

/// command to roll a single d6 die for chance
#[poise::command(slash_command, rename = "lancer-d6", category = "lancer")]
pub async fn lancer_d6(
    ctx: super::Context<'_>,
    #[min_length = 1]
    #[max_length = 128]
    #[rename = "reason"]
    #[description = "short identifier for the reason of this roll"]
    reason: Option<String>,
) -> Result<(), super::Error> {
    // this formula can always be parsed
    let formula = dice::Formula::try_from("d6").unwrap_or_default();

    // generate result for formula
    let result = formula.generate_result();

    // generate message body and reply
    let msg = crate::message::result_message(reason, &result);
    ctx.reply(msg).await?;

    Ok(())
}
