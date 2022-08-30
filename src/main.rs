use crate::util::send_message;
use default::default;
use dotenv::dotenv;
use poise::{
	serenity_prelude::{self as serenity, Ready},
	Framework, PrefixFrameworkOptions,
};
use std::env;

mod util;

type Error = Box<dyn std::error::Error + Send + Sync>;
type BlankResult = std::result::Result<(), Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

/// Leak Ved's IP
#[poise::command(slash_command)]
async fn doxx(ctx: Context<'_>) -> BlankResult {
	ctx.say("Ved's IP is `127.0.0.1`").await?;
	Ok(())
}

/// heheheha
#[poise::command(slash_command)]
async fn heheheha(ctx: Context<'_>) -> BlankResult {
	ctx.say("heheheha").await?;
	for _i in 1..5 {
		send_message(ctx, "heheheha").await?;
	}
	Ok(())
}

/// Register slash commands to discord
#[poise::command(slash_command, prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("Registering commands...").await?;

	let commands = &ctx.framework().options().commands;
	let create_commands = poise::builtins::create_application_commands(commands);
	serenity::Command::set_global_application_commands(ctx.discord(), |b| {
		*b = create_commands; // replace the given builder with the one prepared by poise
		b
	})
	.await?;

	ctx.say("Registered commands").await?;
	Ok(())
}

fn user_data_setup<'a>(
	_ctx: &'a serenity::Context,
	_ready: &'a Ready,
	_framework: &'a Framework<Data, Error>,
) -> Data {
	Data {}
}

#[tokio::main]
async fn main() -> BlankResult {
	dotenv()?;

	println!("Getting DISCORD_API_KEY from .env");
	let discord_api_key = env::var("DISCORD_API_KEY")?;
	println!("Getting youtube API key from .env");
	let _youtube_api_key = env::var("YOUTUBE_API_KEY")?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
	let framework = Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![doxx(), heheheha(), register()],
			prefix_options: PrefixFrameworkOptions {
				prefix: Some("A!".into()),
				case_insensitive_commands: true,
				..default()
			},
			..default()
		})
		.token(discord_api_key)
		.intents(intents)
		.user_data_setup(move |ctx, ready, framework| {
			Box::pin(async move { Ok(user_data_setup(ctx, ready, framework)) })
		});
	println!("Initialized");
	framework.run().await?;

	Ok(())
}