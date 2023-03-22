use crate::command::{Command, CommandInput};
use crate::error::InteractionError;

use async_trait::async_trait;
use twilight_model::application::command::CommandOption;
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{InteractionResponseData};
use twilight_util::builder::InteractionResponseDataBuilder;

pub(crate) struct Help {}

#[async_trait(?Send)]
impl Command for Help {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionResponseData, InteractionError> {
        Ok(InteractionResponseDataBuilder::new()
            .content("Callback message")
            .flags(MessageFlags::EPHEMERAL)
            .build())
    }

    fn name(&self) -> String {
        "help".into()
    }

    fn description(&self) -> String {
        "Information about the bot".into()
    }

    fn options(&self) -> Option<Vec<CommandOption>> {
        None
    }

    async fn autocomplete(
        &self,
        _input: &CommandInput,
    ) -> Result<Option<InteractionResponseData>, InteractionError> {
        Ok(None)
    }
}
