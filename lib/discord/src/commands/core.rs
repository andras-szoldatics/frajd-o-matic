use poise::samples::HelpConfiguration;
use rand::Rng;

/// a hopefully helpful list of commands
#[poise::command(slash_command, rename = "fom-help", category = "core")]
pub async fn help(ctx: super::Context<'_>) -> Result<(), super::Error> {
    let extra_lines = [
        "supported arithmetic entries:",
        "numbers, addition, substraction",
        "",
        "supported dice notation:",
        "d20 2d6 4dF 4d6:H3 2d100:L",
        "",
        "example rolls for /fom-roll:",
        "2d6:H1 - 2d6:L1",
        "d20+2+2d6:H",
    ]
    .join("\n");
    let extra_text_at_bottom = &extra_lines.as_str();

    // generate the list from internal helper
    let config = HelpConfiguration {
        extra_text_at_bottom,
        ..Default::default()
    };
    poise::builtins::help(ctx, None, config).await?;

    Ok(())
}

/// command to roll some dice
#[poise::command(slash_command, rename = "fom-roll", category = "core")]
pub async fn roll(
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

/// command to flip a two-sided coin
#[poise::command(slash_command, rename = "fom-coin", category = "core")]
pub async fn coin_flip(
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
