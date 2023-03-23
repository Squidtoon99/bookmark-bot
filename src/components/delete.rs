use crate::component::{Component, ComponentInput};
use crate::error::InteractionError;

use async_trait::async_trait;
use reqwest::{StatusCode};
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{InteractionResponseType, InteractionResponse};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::console_log;

pub(crate) struct Delete {}

#[async_trait(?Send)]
impl Component for Delete {
    async fn respond(
        &self,
        input: &ComponentInput,
    ) -> Result<InteractionResponse, InteractionError> {
        let data = InteractionResponseDataBuilder::new()
            .content("Bookmark Deleted")
            .flags(MessageFlags::EPHEMERAL)
            .build();
        
        let client = input.http_client()?;

        // Delete message endpoint
        let url = format!("https://discord.com/api/v10/channels/{}/messages/{}", input.channel_id.unwrap().get(), input.message.unwrap().id.get());
        match client.delete(url).send().await {
            Ok(response) => {
                let status = response.status();
                let text: String = response.text().await?;
                console_log!("[DELETE MESSAGE] response: {:?}", text);
                match status {
                    StatusCode::OK => {
                        console_log!("Message deleted");
                    }
                    _ => {
                        console_log!("Error deleting message");
                    }
                }
            }
            Err(e) => {
                console_log!("Error deleting message: {:?}", e);
            }
        }
        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data)
        })
    }

    fn custom_id(&self) -> String {
        "delete".into()
    }
}
