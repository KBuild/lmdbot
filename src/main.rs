use std::env;
use dotenv::dotenv;
use serenity::{prelude::GatewayIntents, Client};

mod handler;
mod bob_generator;
mod role_matcher;

use bob_generator::BobGenerator;
use role_matcher::RoleMatcher;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler::BotHandler { })
        .await
        .expect("Err creating client");
        {
            let mut data = client.data.write().await;
            data.insert::<BobGenerator>(BobGenerator::new("bob_list.txt"));
            data.insert::<RoleMatcher>(RoleMatcher::new("role_map.txt"));
        }
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}