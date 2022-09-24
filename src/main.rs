use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::channel::{Message, Channel};
use serenity::model::prelude::GuildChannel;
use serenity::prelude::*;
use twitter::Twitter;

#[macro_use]
extern crate lazy_static;

mod twitter;

struct Handler {
    launcher: twitter::Twitter,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        let channels = message.guild_id.unwrap().channels(context).await.unwrap();
        let is_target = channels.iter().any(|(_, channel)| channel.name.contains("-rt"));
        let txt = message.content;
        if is_target && txt.matches("twitter.com").count() > 0 {
            #[cfg(debug_assertions)]
            println!("Received twitter triggering message");

            let res = self.launcher.retweet(txt).await;

            #[cfg(debug_assertions)]
            println!("Ran: {:?}", res);
        }
    }

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
                    command.name("help").description("Help! Help! Help!")
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

            let content = match command.data.name.as_str() {
                "ping" => "pong!".to_string(),
                "help" => "이 봇은 테스트중입니다".to_string(),
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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let launcher = Twitter::new(
        env::var("TWITTER_ACCESS_TOKEN").expect("Expected Access Token of Twitter"),
        env::var("TWITTER_REFRESH_TOKEN").expect("Expected Refresh Token of Twitter"),
        env::var("TWITTER_USERID").expect("Expected UserID of Twitter"),
    );

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { launcher })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}