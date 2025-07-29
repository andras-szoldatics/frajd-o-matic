/// command to roll four FATE dice with a modifier
#[poise::command(slash_command, rename = "fate-roll", category = "fate")]
pub async fn fate_roll(
    ctx: super::Context<'_>,
    #[rename = "modifier"]
    #[description = "fixed modifier from approach / skill / other stat"]
    modifier: i64,
    #[min_length = 1]
    #[max_length = 64]
    #[rename = "reason"]
    #[description = "short identifier for the reason of this roll"]
    reason: Option<String>,
) -> Result<(), super::Error> {
    // assemble a string formula by hand
    let dice_formula = format!("4dF{modifier:+}");

    // this formula should always be parsed, but just in case
    let r = dice::Formula::try_from(&dice_formula);
    match r {
        Ok(formula) => {
            // generate result for formula
            let result = formula.generate_result();

            // generate message body and reply
            let msg = crate::message::result_message(reason, &result);
            ctx.reply(msg).await?;
        }
        Err(e) => {
            // generate message body and reply object for ephemeral message
            let msg = crate::message::dice_error_message(&e);
            let reply = poise::CreateReply::default().content(msg).ephemeral(true);

            ctx.send(reply).await?;
        }
    }

    Ok(())
}
