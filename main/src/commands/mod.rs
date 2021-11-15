use anyhow::Result;

use log::error;
use twilight_model::application::interaction::ApplicationCommand;

use crate::types::{Acache, TwHttpClient};

// Make every command a mod
pub mod invite;
pub mod ping;

#[derive(Debug, thiserror::Error)]
pub enum ExecCommandError {
    #[error(r#"The command "{0}" could not be found."#)]
    CommandNotFound(String),
}

pub async fn exec_command(
    http: TwHttpClient,
    command: &ApplicationCommand,
    cache: Acache,
) -> Result<()> {
    let res: Result<(), anyhow::Error> = match command.data.name.as_str() {
        "ping" => ping::execute(http, command, cache).await,
        "invite" => invite::execute(http, command).await,
        _ => Err(anyhow::Error::new(ExecCommandError::CommandNotFound(
            command.data.name.clone(),
        ))),
    };

    match res {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("{:?}", err);
            Err(err)
        }
    }
}
