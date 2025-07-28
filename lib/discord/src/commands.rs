pub mod core;
pub mod fate;
pub mod lancer;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
