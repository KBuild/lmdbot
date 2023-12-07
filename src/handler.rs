use std::env;

use serenity::all::{ResolvedValue, ResolvedOption};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{CommandOptionType, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::bob_generator::BobGenerator;
use crate::role_matcher::RoleMatcher;

pub struct BotHandler { }

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer")
        );

        let ctxdata = ctx.data.as_ref().read().await;
        let role_matcher = ctxdata.get::<RoleMatcher>().unwrap();
        let role_lists = role_matcher.get_all_roles();

        let commands = guild_id.set_commands(&ctx.http, vec![
            CreateCommand::new("ping").description("A ping command"),
            CreateCommand::new("join")
                .description("채널에 참가해 보아요~")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "role",
                        format!("참가하고 싶은 곳의 이름을 적어보아요: {:?}", role_lists.keys())
                    )
                ),
            CreateCommand::new("bob").description("그래서 오늘 뭐 먹음?"),
        ]).await;

        #[cfg(debug_assertions)]
        println!("I now have the following guild slash commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            #[cfg(debug_assertions)]
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => "pong".to_string(),
                "bob" => {
                    let ctxdata = ctx.data.as_ref().read().await;
                    let bob = ctxdata.get::<BobGenerator>().unwrap();
                    bob.pop().to_string()
                },
                "join" => 'return_content: {
                    let member = &mut match command.member.to_owned() {
                        Some(member) => member,
                        None => break 'return_content "참가실패! 프로그래머를 불러봐요~".to_string(),
                    };
                    let option_from_command = command.data.options();
                    let role_name = match option_from_command.first() {
                        Some(ResolvedOption {
                            value: ResolvedValue::String(role_name), ..
                        }) => *role_name,
                        _ => break 'return_content "참가실패! 프로그래머를 불러봐요~".to_string(),
                    };
                    let ctxdata = ctx.data.as_ref().read().await;
                    let role_matcher = ctxdata.get::<RoleMatcher>().unwrap();
                    if let Some(role_id) = role_matcher.get_role_id(role_name) {
                        let _ = member.add_role(&ctx.http, role_id).await;
                        break 'return_content format!("참가성공! : {}", role_name).to_string();
                    }
                    break 'return_content "참가실패! 프로그래머를 불러봐요~".to_string();
                },
                _ => "not implemented :(".to_string(),
            };

            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            #[cfg(debug_assertions)]
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why:#?}");
            };
        }
    }
}
