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
    #[max_length = 64]
    #[rename = "dice-formula"]
    #[description = "dice and fix values to evaluate"]
    dice_formula: String,
    #[min_length = 1]
    #[max_length = 64]
    #[rename = "reason"]
    #[description = "short identifier for the reason of this roll"]
    reason: Option<String>,
) -> Result<(), super::Error> {
    // check if we can parse the formula here
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

/// command to flip a two-sided coin
#[poise::command(slash_command, rename = "fom-coin", category = "core")]
pub async fn coin_flip(ctx: super::Context<'_>) -> Result<(), super::Error> {
    // generate result
    let heads = rand::rng().random();

    // generate message body and reply
    let msg = crate::message::coin_flip_message(heads);
    ctx.reply(msg).await?;

    Ok(())
}
