use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // enable logging for serenity
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("serenity=info"))
        .init();

    // Get the discord token set in `Secrets.toml`
    let discord_token = include_str!("./secret.key").trim();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // core commands
                discord::commands::core::flip_coin(),
                discord::commands::core::roll_dice(),
                // fate commands
                discord::commands::fate::fate_roll(),
                // lancer commands
                discord::commands::lancer::lancer_roll(),
                discord::commands::lancer::lancer_d6(),
                // help commands
                discord::commands::help::commands(),
                discord::commands::help::dice(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(discord::commands::Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}
