pub mod core;
pub mod fate;
pub mod help;
pub mod lancer;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub fn handle_repeats<F>(f: F, repeats: u64) -> String
where
    F: Fn() -> (String, String),
{
    // setup for iteration
    let mut messages = match repeats {
        1 => vec![],
        _ => vec![format!("total # of rolls: {repeats}")],
    };

    for _ in 0..repeats {
        // generate a message
        let (r, f) = f();

        // generate block for result line
        let block = match repeats {
            1 => r.to_string(),
            _ => format!("1. {r}"),
        };
        messages.push(block);

        // generate block for formula line
        if !f.is_empty() {
            let block = match repeats {
                1 => format!("-# {f}"),
                _ => format!("  -# {f}"),
            };
            messages.push(block);
        }
    }

    // generate final body
    messages.join("\n")
}
