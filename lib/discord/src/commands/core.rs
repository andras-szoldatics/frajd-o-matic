use rand::Rng;

/// command to flip a two-sided coin
#[poise::command(slash_command, rename = "flip-coin", category = "core")]
pub async fn flip_coin(
    ctx: super::Context<'_>,
    #[min = 1]
    #[max = 12]
    #[rename = "number-of-flips"]
    #[description = "number of total coin flips to perform, defaults to 1"]
    number_of_flips: Option<u64>,
) -> Result<(), super::Error> {
    // initialize generator closure
    let generator = || {
        let heads = rand::rng().random::<bool>();
        crate::message::coin_flip_message(heads)
    };

    // generate repeated results as reply
    let repeats = number_of_flips.unwrap_or(1);
    let msg = super::handle_repeats(generator, repeats);
    ctx.reply(msg).await?;

    Ok(())
}
/// command to roll some dice
#[poise::command(slash_command, rename = "roll-dice", category = "core")]
pub async fn roll_dice(
    ctx: super::Context<'_>,
    #[min_length = 1]
    #[max_length = 48]
    #[rename = "dice-formula"]
    #[description = "dice and fix values to evaluate"]
    dice_formula: String,
    #[min = 1]
    #[max = 12]
    #[rename = "number-of-rolls"]
    #[description = "number of total rolls to perform, defaults to 1"]
    number_of_rolls: Option<u64>,
    #[min_length = 1]
    #[max_length = 48]
    #[rename = "reason"]
    #[description = "short identifier for the reason of this roll"]
    reason: Option<String>,
) -> Result<(), super::Error> {
    // check if we can parse the formula here
    let r = dice::Formula::try_from(&dice_formula);
    match r {
        Ok(formula) => {
            let repeats = number_of_rolls.unwrap_or(1);

            // initialize generator closure
            let generator = || {
                let result = formula.generate_result();
                crate::message::result_message(result, reason.as_ref(), repeats)
            };

            // generate repeated results as reply
            let msg = super::handle_repeats(generator, repeats);
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
