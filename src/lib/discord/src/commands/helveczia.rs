const STATS: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];

/// command to generate a stat block for Helvéczia RPG
#[poise::command(slash_command, rename = "helveczia-stats", category = "helveczia")]
pub async fn stat_block(ctx: super::Context<'_>) -> Result<(), super::Error> {
    // assemble a string formula by hand
    let dice_formula = String::from("4d6:H3");

    // this formula should always be parsable, but just in case
    let r = dice::Formula::try_from(&dice_formula);
    let formula = match r {
        Ok(formula) => formula,
        Err(e) => {
            // generate message body and reply object for ephemeral message
            let msg = crate::message::dice_error_message(&e);
            let reply = poise::CreateReply::default().content(msg).ephemeral(true);

            ctx.send(reply).await?;

            return Ok(());
        }
    };

    let mut messages = vec![String::from("stat block for Helvéczia")];
    // assemble a stat block message
    for stat in STATS {
        let result = formula.generate_result();
        let reason = Some(stat.to_string());
        let (r, f) = crate::message::result_message(&result, reason.as_ref(), 1);

        // filter for empty lines on return
        if !r.is_empty() {
            messages.push(r);
        }

        if !f.is_empty() {
            let block = format!("-# {f}");
            messages.push(block);
        }
    }

    // send back the compiled message
    let msg = messages.join("\n");
    ctx.reply(msg).await?;

    Ok(())
}
