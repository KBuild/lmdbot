use std::env;

use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::{ResolvedValue, ResolvedOption};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage};
use serenity::client::{Context, EventHandler};
use serenity::model::application::{CommandOptionType, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::Message;

use crate::bob_generator::BobGenerator;
use crate::lotto_generator::LottoGenerator;
use crate::role_matcher::RoleMatcher;

pub struct BotHandler { 
    pub use_twitter_replacer: bool,
}

lazy_static! {
    pub static ref X_TWITTER_MATCH: Regex = Regex::new(r"https\:\/\/(twitter|x)(.com\/\w+\/status\/\d+)").unwrap();
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if self.use_twitter_replacer && X_TWITTER_MATCH.is_match(&msg.content) {
            let new_content = X_TWITTER_MATCH.replace(&msg.content, "https://fxtwitter$2");
            let original_user_id = msg.author.id;
            let builder = CreateMessage::new().content(format!("[링크수정] <@{}>\n{}", original_user_id, new_content));
            msg.channel_id.send_message(&ctx.http, builder).await.unwrap();
            msg.channel_id.delete_message(&ctx.http, msg.id).await.unwrap();   
        }
    }

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
            CreateCommand::new("lotto").description("로또번호 생성"),
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
                "lotto" => {
                    let ctxdata = ctx.data.as_ref().read().await;
                    let lotto_gen = ctxdata.get::<LottoGenerator>().unwrap();
                    lotto_gen.choose_lotto_6_45()
                }
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
