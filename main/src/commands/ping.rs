use anyhow::Result;
use twilight_model::application::{callback::InteractionResponse, interaction::ApplicationCommand};
use twilight_util::builder::CallbackDataBuilder;
use twilight_util::snowflake::Snowflake;

use crate::types::{Acache, TwHttpClient};

pub async fn execute(
    http: TwHttpClient,
    command: &ApplicationCommand,
    cache: Acache,
) -> Result<()> {
    // if command.user.as_ref().unwrap().id.0 != UserId::new(615542460151496705_u64).unwrap().0 {
    //     return Ok(());
    // }

    let ping_time = chrono::Utc::now().timestamp_millis() - command.id.timestamp();

    let guild_count = cache.iter().guilds().count();

    http.interaction_callback(
        command.id,
        &command.token,
        &InteractionResponse::ChannelMessageWithSource(
            CallbackDataBuilder::new()
                .content(format!(
                    "PONG! Ping time: {}ms\nCached Guilds: {}",
                    ping_time, guild_count
                ))
                .build(),
        ),
    )
    .exec()
    .await?;

    Ok(())
}
