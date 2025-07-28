use rand::{Rng, rng};

fn process_dice(dice: &crate::Dice) -> crate::Value {
    let mut value = crate::Value::default();

    // retrieve thread based RNG
    let mut trng = rng();

    for _ in 0..dice.count {
        // generate sub-results
        let roll = match dice.die {
            crate::Die::Number(n) => match n {
                0 => 0,
                _ => trng.random_range(1..=n),
            },
            crate::Die::Fate => trng.random_range(-1..=1),
        };

        // add roll to values as kept
        value.sub_values.push((roll, true));
    }

    match dice.keep {
        crate::Keep::All => (), // no-op
        crate::Keep::Highest(n) => {
            let cycles = value.sub_values.len() as u64 - n;

            for _ in 0..cycles {
                let mut lowest_value = i64::MAX;

                // find the lowest active value
                for (value, active) in &value.sub_values {
                    if *active && *value < lowest_value {
                        lowest_value = *value;
                    }
                }

                // iterate backwards and remove the first matching value
                for (value, active) in value.sub_values.iter_mut().rev() {
                    if *active && *value == lowest_value {
                        *active = false;
                        break;
                    }
                }
            }
        }
        crate::Keep::Lowest(n) => {
            let cycles = value.sub_values.len() as u64 - n;

            for _ in 0..cycles {
                let mut highest_value = i64::MIN;

                // find the highest active value
                for (value, active) in &value.sub_values {
                    if *active && *value > highest_value {
                        highest_value = *value;
                    }
                }

                // iterate backwards and remove the first matching value
                for (value, active) in value.sub_values.iter_mut().rev() {
                    if *active && *value == highest_value {
                        *active = false;
                        break;
                    }
                }
            }
        }
    }

    // summarize active values
    let total = value
        .sub_values
        .iter()
        .filter(|(_, active)| *active)
        .map(|(value, _)| value)
        .sum();
    value.final_value = Some(total);

    value
}

fn process_number(number: i64) -> crate::Value {
    crate::Value {
        final_value: Some(number),
        ..Default::default()
    }
}

pub fn process_entry(entry: &crate::Entry) -> crate::Value {
    match &entry.category {
        crate::Category::Dice(dice) => process_dice(dice),
        crate::Category::Number(number) => process_number(*number),
        _ => crate::Value::default(),
    }
}
