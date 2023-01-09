use std::env;
use dotenv::dotenv;
use serenity::{prelude::GatewayIntents, Client};

mod handler;
mod bob_generator;

use bob_generator::BobGenerator;

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
        }
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}