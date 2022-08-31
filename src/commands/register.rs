use crate::{BlankResult, Context};
use poise::serenity_prelude::Command;

/// Register slash commands to discord
#[poise::command(slash_command, prefix_command)]
pub async fn register(ctx: Context<'_>) -> BlankResult {
	let message = ctx.say("Registering commands...").await?;

	let commands = &ctx.framework().options().commands;
	let create_commands = poise::builtins::create_application_commands(commands);
	Command::set_global_application_commands(ctx.discord(), |b| {
		*b = create_commands; // replace the given builder with the one prepared by poise
		b
	})
	.await?;

	message
		.edit(ctx, |m| m.content("Registered commands."))
		.await?;

	Ok(())
}
