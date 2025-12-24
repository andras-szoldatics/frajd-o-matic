use poise::samples::HelpConfiguration;

/// shows a list of all available commands and their descriptions
#[poise::command(slash_command, rename = "help-commands", category = "help")]
pub async fn commands(ctx: super::Context<'_>) -> Result<(), super::Error> {
    // generate the list from internal helper
    let config = HelpConfiguration {
        ..Default::default()
    };
    poise::builtins::help(ctx, None, config).await?;

    Ok(())
}

/// shows a detailed explanation of the dice notation syntax
#[poise::command(slash_command, rename = "help-dice", category = "help")]
pub async fn dice(ctx: super::Context<'_>) -> Result<(), super::Error> {
    let help_lines = vec![
        "flexible dice notation for tabletop-style rolling, combine numbers, dice, and arithmetic operators",
        "### supported entries",
        "- numbers: any positive integer (e.g. `2`, `10`, `100`)",
        "- addition and subtraction: `+` and `-` to add or subtract numbers or dice results (e.g. `d20 + 2`, `7 - 2d6`)",
        "- dice: number of dice should be at most 100, number of sides at most 1000:",
        "  - standard dice: `d20`, `2d6`, `4D100` (number of dice and sides)",
        "  - fate dice: `4dF` (four FATE dice, each -1, 0, or +1)",
        "  - omit number before `d` to roll one die (e.g. `d6` = `1d6`)",
        "### keep highest/lowest",
        "- keep highest or lowest dice using `:H` or `:L` (case-insensitive):",
        "  - `4d6:H3` - four d6, keep highest three",
        "  - `2d100:L` - two d100, keep lowest one",
        "  - omit number: defaults to 1 (e.g. `2d6:H` = keep highest one)",
        "  - if number to keep >= dice rolled, all dice are kept",
        "### examples",
        "- `d20 + 2` - one d20 plus 2",
        "- `2d6:h - 2d6:l` - two d6 and keep the highest one, subtract lowest one from another roll",
        "- `d20+2+2d6:H` - d20, plus 2, plus highest of two d6",
        "- `4dF+3` - four FATE dice plus 3",
        "- `2d100:L` - two d100, keep lowest",
    ];

    // generate message body & reply
    let help_text = help_lines.join("\n");
    let reply = poise::CreateReply::default()
        .content(help_text)
        .ephemeral(true);
    ctx.send(reply).await?;

    Ok(())
}
