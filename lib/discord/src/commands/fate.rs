/// command to roll four FATE dice with a modifier
#[poise::command(slash_command, rename = "fate-roll", category = "fate")]
pub async fn fate_roll(
    ctx: super::Context<'_>,
    #[rename = "modifier"]
    #[description = "fixed modifier from approach / skill / other stat"]
    modifier: i64,
    #[min = 1]
    #[max = 12]
    #[rename = "number-of-rolls"]
    #[description = "number of total rolls to perform, defaults to 1"]
    number_of_rolls: Option<u64>,
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
            // initialize generator closure
            let generator = || {
                let result = formula.generate_result();
                crate::message::result_message(reason.as_ref(), &result)
            };

            // generate repeated results as reply
            let msg = super::handle_repeats(number_of_rolls, generator);
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
