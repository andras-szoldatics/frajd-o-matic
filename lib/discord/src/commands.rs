pub mod core;
pub mod fate;
pub mod lancer;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub fn handle_repeats<F>(repeats: Option<u64>, f: F) -> String
where
    F: Fn() -> (String, String),
{
    // setup for iteration
    let number_of_sets = repeats.unwrap_or(1);
    let mut messages = match number_of_sets {
        1 => vec![],
        _ => vec![format!("total # of rolls: {number_of_sets}")],
    };

    for _ in 0..number_of_sets {
        // generate a message
        let (r, f) = f();

        // generate block for result line
        let block = match number_of_sets {
            1 => r.to_string(),
            _ => format!("1. {r}"),
        };
        messages.push(block);

        // generate block for formula line
        if !f.is_empty() {
            let block = match number_of_sets {
                1 => format!("-# {f}"),
                _ => format!("  -# {f}"),
            };
            messages.push(block);
        }
    }

    // generate final body
    messages.join("\n")
}
