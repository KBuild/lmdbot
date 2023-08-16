use std::env;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;

use crate::bob_generator::BobGenerator;
use crate::role_matcher::RoleMatcher;

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

        let ctxdata = ctx.data.as_ref().read().await;
        let role_matcher = ctxdata.get::<RoleMatcher>().unwrap();
        let role_lists = role_matcher.get_all_roles();

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("join").description("채널에 참가해 보아요~")
                        .create_option(|subcommand| {
                            subcommand
                                .kind(CommandOptionType::String)
                                .name("role")
                                .description(format!("참가하고 싶은 곳의 이름을 적어보아요: {:?}", role_lists.keys()))
                        })
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

            let data = command.data.to_owned();
            match data.name.as_str() {
                "ping" => {
                    let _ = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content("pong"))
                        })
                        .await;
                },
                "bob" => {
                    let ctxdata = ctx.data.as_ref().read().await;
                    let bob = ctxdata.get::<BobGenerator>().unwrap();
                    let response_msg = bob.pop().to_string();
                    let _ = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(response_msg))
                        })
                        .await;
                },
                "join" => {
                    let member = &mut match command.member.to_owned() {
                        Some(member) => member,
                        None => return,
                    };
                    let first_option = match data.options.first() {
                        Some(opt) => opt,
                        None => return,
                    };
                    let role_name = match &first_option.value {
                        Some(option) => if option.is_string() { option.as_str().unwrap() } else { "" },
                        None => return,
                    };

                    let ctxdata = ctx.data.as_ref().read().await;
                    let role_matcher = ctxdata.get::<RoleMatcher>().unwrap();
                    if let Some(role_id) = role_matcher.get_role_id(role_name) {
                        let _ = member.add_role(&ctx.http, role_id).await;
                        let response_msg = format!("참가성공! : {}", role_name);
                        let _ = command
                            .create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|message| message.content(response_msg))
                            })
                            .await;
                        return;
                    }
                    let response_msg = "참가실패! 프로그래머를 불러봐요~".to_string();
                    let _ = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(response_msg))
                        })
                        .await;
                },
                _ => (),
            }
        }
    }
}
