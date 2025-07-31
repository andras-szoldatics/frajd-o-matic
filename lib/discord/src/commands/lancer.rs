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
    let dice_formula = if sixes == 0 {
        format!("d20{modifier:+}")
    } else {
        format!("d20{modifier:+}{sixes:+}d6:H")
    };

    // this formula should always be parsable, but just in case
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

/// command to roll a single d6 die for chance
#[poise::command(slash_command, rename = "lancer-d6", category = "lancer")]
pub async fn lancer_d6(
    ctx: super::Context<'_>,
    #[min_length = 1]
    #[max_length = 64]
    #[rename = "reason"]
    #[description = "short identifier for the reason of this roll"]
    reason: Option<String>,
) -> Result<(), super::Error> {
    // assemble a string formula by hand
    let dice_formula = "D6";

    // this formula should always be parsable, but just in case
    let r = dice::Formula::try_from(dice_formula);
    match r {
        Ok(formula) => {
            // initialize generator closure
            let generator = || {
                let result = formula.generate_result();
                crate::message::result_message(result, reason.as_ref(), 1)
            };

            // generate repeated results as reply
            let msg = super::handle_repeats(generator, 1);
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
