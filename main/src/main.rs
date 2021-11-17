use bot::{
    constants::{
        API_DOCS_BOT_ID, API_DOCS_CHANNEL, ISSUE_BUTTON_EMOJI, ISSUE_MANAGEMENT_USERS,
        REMOVE_BUTTON_EMOJI,
    },
    github::create_issues,
    types::TwHttpClient,
};
use dotenv::dotenv;
use futures::stream::StreamExt;
use std::{env, error::Error, sync::Arc};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event,
};
use twilight_http::Client as HttpClient;
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        component::button::ButtonStyle,
        interaction::Interaction,
    },
    channel::message::MessageFlags,
    gateway::{payload::incoming::MessageCreate, Intents},
    id::{ApplicationId, MessageId, UserId},
};
use util::builder::{ButtonBuilder, ComponentBuilder};

// TODO: look at this cool thing when its finished https://github.com/baptiste0928/twilight-interactions

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Load the .env file and just ignore any errors
    dotenv().ok();
    env_logger::init();

    info!("Starting up");

    let token = env::var("DISCORD_TOKEN")?;

    // This is the default scheme. It will automatically create as many
    // shards as is suggested by Discord.
    let scheme = ShardScheme::Auto;

    // Use intents to only receive guild message events.
    let (cluster, mut events) =
        Cluster::builder(token.to_owned(), Intents::GUILDS | Intents::GUILD_MESSAGES)
            .shard_scheme(scheme)
            .build()
            .await?;
    let cluster = Arc::new(cluster);

    // Start up the cluster.
    let cluster_spawn = Arc::clone(&cluster);

    // Start all shards in the cluster in the background.
    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token));
    http.set_application_id(
        ApplicationId::new(906182472507740161_u64)
            .expect("Could not create Application id for http"),
    );

    // Process each event as they come in.
    while let Some((shard_id, event)) = events.next().await {
        tokio::spawn(handle_event(shard_id, event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: TwHttpClient,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(mut message) => {
            if message.content.starts_with("++fakeit")
                && message.author.id == UserId::new(615542460151496705_u64).unwrap()
            {
                let id = if let Some(id) = message.content.split(' ').nth(1) {
                    let raw = id.parse::<u64>()?;
                    let id = MessageId::new(raw);
                    if id.is_none() {
                        return Ok(());
                    }

                    id.unwrap()
                } else {
                    return Ok(());
                };

                let to_fake = http
                    .message(API_DOCS_CHANNEL, id)
                    .exec()
                    .await?
                    .model()
                    .await?;

                message = Box::new(MessageCreate(to_fake));
            }

            if message.channel_id == API_DOCS_CHANNEL || message.content.starts_with("++fakeit") {
                // Messages could be send from someone else so check the author
                if message.author.id != API_DOCS_BOT_ID {
                    return Ok(());
                }

                let components = ComponentBuilder::new()
                    .button(
                        ButtonBuilder::new(ButtonStyle::Primary, "create-github-issue".into())
                            .emoji(ISSUE_BUTTON_EMOJI)
                            .build(),
                    )
                    .button(
                        ButtonBuilder::new(ButtonStyle::Secondary, "delete-message".into())
                            .emoji(REMOVE_BUTTON_EMOJI)
                            .build(),
                    )
                    .build();

                http.create_message(API_DOCS_CHANNEL)
                    .embeds(&message.embeds)?
                    .components(&components)?
                    .exec()
                    .await?;

                http.delete_message(API_DOCS_CHANNEL, message.id)
                    .exec()
                    .await?;
            }
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        Event::InteractionCreate(interaction) => {
            if let Interaction::MessageComponent(component) = interaction.0 {
                match component.data.custom_id.as_str() {
                    "create-github-issue" => {
                        let perms = match component.member {
                            Some(member) if member.user.is_some() => {
                                ISSUE_MANAGEMENT_USERS.contains(&member.user.unwrap().id)
                            }
                            _ => false,
                        };

                        if !perms {
                            http.interaction_callback(
                                component.id,
                                &component.token,
                                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                                    allowed_mentions: None,
                                    components: None,
                                    content: Some("You do not have access to this.".into()),
                                    embeds: vec![],
                                    flags: Some(MessageFlags::EPHEMERAL),
                                    tts: None,
                                }),
                            )
                            .exec()
                            .await?;

                            return Ok(());
                        }

                        // The message must have an embed at this point so its safe to use.
                        if let Some(url) = &component.message.embeds[0].url {
                            http.interaction_callback(
                                component.id,
                                &component.token,
                                &InteractionResponse::DeferredUpdateMessage,
                            )
                            .exec()
                            .await?;

                            create_issues(url.clone()).await?;

                            http.update_interaction_original(&component.token)?
                                .components(Some(&[]))?
                                .exec()
                                .await?;

                            return Ok(());
                        }

                        http.interaction_callback(
                            component.id,
                            &component.token,
                            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                                allowed_mentions: None,
                                components: None,
                                content: Some("Could not get the issue link.".into()),
                                embeds: vec![],
                                flags: Some(MessageFlags::EPHEMERAL),
                                tts: None,
                            }),
                        )
                        .exec()
                        .await?;
                    }
                    "delete-message" => {
                        let perms = match component.member {
                            Some(member) if member.user.is_some() => {
                                ISSUE_MANAGEMENT_USERS.contains(&member.user.unwrap().id)
                            }
                            _ => false,
                        };

                        if !perms {
                            http.interaction_callback(
                                component.id,
                                &component.token,
                                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                                    allowed_mentions: None,
                                    components: None,
                                    content: Some("You do not have access to this.".into()),
                                    embeds: vec![],
                                    flags: Some(MessageFlags::EPHEMERAL),
                                    tts: None,
                                }),
                            )
                            .exec()
                            .await?;

                            return Ok(());
                        }

                        http.interaction_callback(
                            component.id,
                            &component.token,
                            &InteractionResponse::UpdateMessage(CallbackData {
                                allowed_mentions: None,
                                components: Some(vec![]),
                                content: None,
                                embeds: vec![],
                                flags: None,
                                tts: None,
                            }),
                        )
                        .exec()
                        .await?;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    Ok(())
}
