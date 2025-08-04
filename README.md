# frajd-o-matic

Handmade discord dice bot written in rust.

This project utilizes the poise [[link](https://docs.rs/poise/latest/poise/)] and serenity [[link](https://docs.rs/serenity/latest/serenity/)] frameworks for wrapping the Discord API.

This project also utilizes the shuttle_runtime [[link](https://docs.rs/shuttle-runtime/latest/shuttle_runtime/)] crate and the shuttle platform for deployment.

## available slash commands

### core commands

- `/coins` — command to flip a two-sided coin
  - `number-of-flips` (integer, optional, 1-12): number of total coin flips to perform, defaults to 1
- `/dice` — command to roll some dice
  - `dice-formula` (string, required, 1-48 chars): dice and fixed values to evaluate, e.g. `d20+2+2d6:H`
  - `number-of-rolls` (integer, optional, 1-12): number of total rolls to perform, defaults to 1
  - `reason` (string, optional, 1-48 chars): short identifier for the reason, e.g. `attack the goblin`

### fate commands

- `/fate-roll` — shortcut to roll four FATE dice with a modifier
  - `modifier` (integer, required): fixed modifier from approach, skill, or other stat
  - `number-of-rolls` (integer, optional, 1-12): number of total rolls to perform, defaults to 1
  - `reason` (string, optional, 1-64 chars): short identifier for the reason, e.g. `create advantage`

### lancer commands

- `/lancer-roll` — shortcut to roll a Lancer attack, check, or save
  - `modifier` (integer, required): fixed modifier from mech skill, trigger, or other stat
  - `accuracy-or-difficulty` (integer, required): degree of total accuracy or difficulty
    - `0`: normal roll
    - positive: number of accuracy dice (e.g. `+2` for 2 accuracy)
    - negative: number of difficulty dice (e.g. `-1` for 1 difficulty)
  - `number-of-rolls` (integer, optional, 1-12): number of total rolls to perform, defaults to 1
  - `reason` (string, optional, 1-64 chars): short identifier for the reason, e.g. `attack the mech`
- `/lancer-d6` — shortcut to roll a single d6 die for chance / recharge
  - `reason` (string, optional, 1-64 chars): short identifier for the reason, e.g. `NPC recharge`

### help commands

- `/help-commands` — shows a list of all available commands and their descriptions, sent as private message
- `/help-dice` — shows a detailed explanation of the dice notation syntax, sent as private message

## available dice notation

flexible dice notation for tabletop-style rolling, combine numbers, dice, and arithmetic operators

### supported entries

- numbers: any positive integer (e.g. `2`, `10`, `100`)
- addition and subtraction: `+` and `-` to add or subtract numbers or dice results (e.g. `d20 + 2`, `7 - 2d6`)
- dice, number of dice should be at most 100, number of sides at most 1000:
  - standard dice: `d20`, `2d6`, `4D100` (number of dice and sides)
  - fate dice: `4dF` (four FATE dice, each -1, 0, or +1)
  - omit number before `d` to roll one die (e.g. `d6` = `1d6`)

### keep highest/lowest

- keep highest or lowest dice using `:H` or `:L` (case-insensitive):
  - `4d6:H3` - four d6, keep highest three
  - `2d100:L` - two d100, keep lowest one
  - omit number: defaults to 1 (e.g. `2d6:H` = keep highest one)
  - if number to keep >= dice rolled, all dice are kept

### examples

- `d20 + 2` - one d20 plus 2
- `2d6:h - 2d6:l` - two d6 and keep the highest one, subtract lowest one from another roll
- `d20+2+2d6:H` - d20, plus 2, plus highest of two d6
- `4dF+3` - four FATE dice plus 3
- `2d100:L` - two d100, keep lowest

### error handling

- invalid or malformed formulas return an error message with problem and position

## parameters required to run or deploy

you need a `Secrets.toml` file in the root of the project with:

```toml
DISCORD_TOKEN = 'your_discord_token_here'
```

## license

This project is licensed under the MIT license. you are free to use, modify, and distribute it as you like, as long as you include the original license file.

For full details, see [[LICENSE](./LICENSE)].
