use std::env;

use serenity::async_trait;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::bob_generator::BobGenerator;
struct GlueEngine { }

pub struct BotHandler { }

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("join").description("채널에 참가해 보아요~")
                })
                .create_application_command(|command| {
                    command.name("bob").description("그래서 오늘 뭐 먹음?")
                })
        })
        .await;

        #[cfg(debug_assertions)]
        println!("I now have the following guild slash commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            #[cfg(debug_assertions)]
            println!("Received command interaction: {:#?}", command);

            let data = ctx.data.read().await;
            let bob = data.get::<BobGenerator>().unwrap();

            let content = match command.data.name.as_str() {
                "ping" => "pong!".to_string(),
                "join" => "".to_string(),
                "bob" => bob.pop().clone(),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
