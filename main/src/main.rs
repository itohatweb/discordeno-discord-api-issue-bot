use bot::types::{
    github::{Commit, GitHubCommit, GitHubCompare},
    TwHttpClient,
};
use dotenv::dotenv;
use futures::stream::StreamExt;
use std::{env, error::Error, sync::Arc, thread, time::Duration};
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
    channel::{message::MessageFlags, ReactionType},
    gateway::{payload::incoming::MessageCreate, Intents},
    id::{ApplicationId, ChannelId, EmojiId, MessageId, UserId},
};
use util::builder::{ButtonBuilder, ComponentBuilder};

const API_DOCS_CHANNEL: ChannelId = unsafe { ChannelId::new_unchecked(881991954676715653_u64) };
const API_DOCS_BOT_ID: UserId = unsafe { UserId::new_unchecked(881992163855065089_u64) };
const ISSUE_MANAGEMENT_USERS: [UserId; 1] =
    unsafe { [UserId::new_unchecked(615542460151496705_u64)] };

const ISSUE_BUTTON_EMOJI: ReactionType = unsafe {
    ReactionType::Custom {
        animated: false,
        id: EmojiId::new_unchecked(754789242412073010_u64),
        name: None,
    }
};

const REMOVE_BUTTON_EMOJI: ReactionType = unsafe {
    ReactionType::Custom {
        animated: false,
        id: EmojiId::new_unchecked(853559407027683328_u64),
        name: None,
    }
};

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
                let id = if let Some(id) = message.content.split(" ").skip(1).next() {
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

                // http.delete_message(API_DOCS_CHANNEL, message.id)
                //     .exec()
                //     .await?;
            }
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        Event::VoiceStateUpdate(_vsu) => {
            // println!("vsu: {:?}", vsu);
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

#[derive(Debug, thiserror::Error)]
enum CreateIssuesError {
    #[error("No Hash could be found.")]
    NoHashFound,
    #[error("Error from reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

async fn create_issues(url: String) -> Result<(), CreateIssuesError> {
    fn build_request_headers(mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request = request.header(reqwest::header::USER_AGENT, "Discordeno Issue Creation Bot");
        request = request.header(
            reqwest::header::AUTHORIZATION,
            format!("token {}", env::var("GITHUB_ACCESS_TOKEN").unwrap()),
        );

        request
    }

    if let Ok(client) = reqwest::Client::builder().build() {
        let hash_part = url.split("/").last();
        if hash_part == None {
            return Err(CreateIssuesError::NoHashFound);
        }

        let mut commits: Vec<Commit> = vec![];

        // Check whether its a normal commit url or an url to a compare.
        if !hash_part.unwrap().contains("...") {
            // Its a normal commit URL
            // First get the issue
            let mut request = client.get(format!(
                "https://api.github.com/repos/discord/discord-api-docs/commits/{}",
                hash_part.unwrap()
            ));

            request = build_request_headers(request);
            let res = request.send().await;
            let commit_data = res.unwrap().json::<GitHubCommit>().await?;
            commits.push(commit_data.commit);
        } else {
            // Its a compare URL
            // First get the issue
            let mut request = client.get(format!(
                "https://api.github.com/repos/discord/discord-api-docs/compare/{}",
                hash_part.unwrap()
            ));

            request = build_request_headers(request);
            let res = request.send().await;
            let compare_data = res.unwrap().json::<GitHubCompare>().await?;
            commits = compare_data
                .commits
                .into_iter()
                .map(|cmp| cmp.commit)
                .collect();
        }

        let mut len = commits.len();

        for commit in commits {
            let url = match commit.url.split("/").last() {
                Some(hash) => format!(
                    "https://github.com/discord/discord-api-docs/commit/{}",
                    hash
                ),
                None => commit.url,
            };

            let mut request =
                client.post("https://api.github.com/repos/discordeno/discordeno/issues");

            request = build_request_headers(request);
            request = request.header(reqwest::header::ACCEPT, "application/vnd.github.v3+json");
            request
                .json(&GithubCreateIssue::new(commit.message, url))
                .send()
                .await?;

            len -= 1;
            if len != 0 {
                thread::sleep(Duration::from_secs(5));
            }
        }
    }

    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct GithubCreateIssue {
    title: String,
    body: String,
    labels: Vec<String>,
}

impl GithubCreateIssue {
    fn new(commit_message: String, url: String) -> Self {
        Self {
            title: format!("[api-docs] {}", commit_message),
            body: format!(
                "A new commit was made into the api-docs repo.\n{}\n\nThis is a bot created issue.",
                url
            ),
            labels: vec!["api-docs-commits".into()],
        }
    }
}

// /// Job given to workers.
// #[derive(Clone)]
// enum Job {
//     CreateIssue(String),
// }

// /// Message sent to workers.
// #[derive(Clone)]
// enum Message {
//     AddJob(Job),
//     Quit,
// }

// use crossbeam_channel::{unbounded, Receiver, Sender};

// struct Worker<M> {
//     tx: Sender<M>,
//     _rx: Receiver<M>,
//     handle: JoinHandle<()>,
// }

// impl Worker<Message> {
//     fn add_job(&self, job: Job) {
//         self.tx
//             .send(Message::AddJob(job))
//             .expect("failed to add job");
//     }
//     fn join(self) {
//         self.handle.join().expect("failed to join thread");
//     }
//     fn send_msg(&self, msg: Message) {
//         self.tx.send(msg).expect("failed to send message");
//     }
// }

// #[derive(serde::Serialize)]
// struct GithubIssueBody {
//     title: String,
//     body: String,
//     labels: String,
// }

// impl GithubIssueBody {
//     fn new(commit_message: String, url: String) -> Self {
//         Self {
//             title: format!("[api-docs] {}", commit_message),
//             body: format!("A new commit was made into the api-docs repo.\n{}", url),
//             labels: "api-docs-commits".into(),
//         }
//     }
// }

// /// Create a new worker to receive jobs.
// fn spawn_worker() -> Worker<Message> {
//     let (tx, rx) = unbounded();
//     // We clone the receiving end here so we have a copy to give to the
//     // thread. This allows us to save the `tx` and `rx` into the Worker struct.
//     let rx_thread = rx.clone();
//     // Spawn a new thread.
//     let handle = thread::spawn(move || {
//         // VecDeque allows us to get jobs in the order they arrive.
//         let mut jobs = VecDeque::new();

//         fn build_request_headers(mut request: RequestBuilder) -> RequestBuilder {
//             request = request.header(reqwest::header::USER_AGENT, "Discordeno Issue Creation Bot");

//             request = request.basic_auth(
//                 env::var("GITHUB_CLIENT_ID").unwrap(),
//                 Some(env::var("GITHUB_APP_TOKEN").unwrap()),
//             );

//             request
//         }

//         loop {
//             // Get the next job.
//             if let Some(job) = jobs.pop_front() {
//                 use Job::*;
//                 match job {
//                     CreateIssue(url) => {
//                         if let Ok(client) = reqwest::blocking::Client::builder().build() {
//                             let hash_part = url.split("/").last();
//                             if hash_part == None {
//                                 continue;
//                             }

//                             let commits: Vec<Commit> = vec![];

//                             // Check whether its a normal commit url or an url to a compare.
//                             if !hash_part.unwrap().contains("...") {
//                                 // Its a normal commit URL
//                                 // First get the issue
//                                 let mut request = client.get(format!(
//                                     "https://api.github.com/repos/discord/discord-api-docs/issue/{}",
//                                     hash_part.unwrap()
//                                 ));

//                                 request = build_request_headers(request);
//                                 let res = request.send();
//                                 let commit_data = res.unwrap().json::<GitHubCommit>();
//                                 error!("coomm, {:#?}", commit_data);
//                             } else {
//                                 // Its a compare URL
//                             }
//                             // let commit_data = res.unwrap().text();

//                             // let mut request = client
//                             //     .post("https://api.github.com/repos/discordeno/discordeno/issues");
//                             // request = request.header(
//                             //     reqwest::header::USER_AGENT,
//                             //     "Discord API Commits issue creation bot for discordeno",
//                             // );
//                             // request = request.basic_auth(
//                             //     env::var("GITHUB_CLIENT_ID").unwrap(),
//                             //     Some(env::var("GITHUB_APP_TOKEN").unwrap()),
//                             // );
//                             // request.body(None);
//                         }

//                         // Wait 5 secs cuz rate limits
//                         thread::sleep(Duration::from_secs(5));
//                     }
//                 }
//             }

//             // Check for messages on the channel.
//             if let Ok(msg) = rx_thread.try_recv() {
//                 match msg {
//                     Message::AddJob(job) => {
//                         // When we receive a new job, add it
//                         // to the jobs list.
//                         jobs.push_back(job);
//                         // Continue processing jobs.
//                         continue;
//                     }
//                     Message::Quit => return,
//                 }
//             } else {
//                 // No messages on the channel,
//                 // pause to wait for more messages to arrive on channel.
//                 thread::sleep(Duration::from_millis(100));
//             }
//         }
//     });

//     Worker {
//         tx,
//         _rx: rx,
//         handle,
//     }
// }
