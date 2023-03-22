use crate::component::{Component as ComponentTrait, ComponentInput};
use crate::error::InteractionError;
use crate::input::SharedInput;

use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle},
    Component,
};

use async_trait::async_trait;
use twilight_model::channel::message::{MessageFlags, ReactionType};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::console_log;

pub(crate) struct Color {}

#[async_trait(?Send)]
impl ComponentTrait for Color {
    async fn respond(
        &self,
        input: &ComponentInput,
    ) -> Result<InteractionResponse, InteractionError> {
        let custom_id: String = input.custom_id.clone();

        // If a colon exists in the custom_id then the user has picked a color, otherwise show the color picker

        if custom_id.contains(":") {
            let (_name, data) = custom_id.split_once(":").unwrap();
            let (color, url) = data.split_once(":").unwrap();
            let url = format!("https://discord.com/channels/{}", url);
            console_log!("color: {}, url: {}", color, url);
            let msg = input.message.unwrap();
            let mut embeds = msg.embeds.clone();

            for mut embed in embeds.iter_mut() {
                embed.color = Some(color.parse::<u32>().unwrap());
            }

            let default_components = input.default_components(url);

            Ok(InteractionResponse {
                kind: InteractionResponseType::UpdateMessage,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .embeds(embeds)
                        .components(default_components)
                        .content("")
                        .build(),
                ),
            })
        } else {
            let Some(Component::ActionRow(row)) = input.message.unwrap().components.last() else {
                    return Err(InteractionError::WorkerError("No components found".to_string()))
                };
            let Some(Component::Button(Button {url, ..})) = row.components.last() else {
                    return Err(InteractionError::WorkerError("No components found".to_string()))
                };
            let url = {
                let u = url.as_ref().unwrap();
                let (_, data) = u.split_at(27);
                data.to_string()
            };
            let components = Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        style: ButtonStyle::Primary,
                        label: Some("Blurple".into()),
                        custom_id: Some(format!("color:5793266:{url}", url = url)),
                        emoji: None,
                        url: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: Some("Red".into()),
                        custom_id: Some(format!("color:15548997:{url}", url = url)),
                        emoji: None,
                        url: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Primary,
                        label: Some("Green".into()),
                        custom_id: Some(format!("color:5763719:{url}", url = url)),
                        emoji: None,
                        url: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: Some("Yellow".into()),
                        custom_id: Some(format!("color:16705372:{url}", url = url)),
                        emoji: None,
                        url: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Primary,
                        label: Some("White".into()),
                        custom_id: Some(format!("color:16777215:{url}", url = url)),
                        emoji: None,
                        url: None,
                        disabled: false,
                    }),
                ],
            });

            let data = InteractionResponseDataBuilder::new()
                .components([components])
                .build();

            Ok(InteractionResponse {
                kind: InteractionResponseType::UpdateMessage,
                data: Some(data),
            })
        }
    }

    fn custom_id(&self) -> String {
        "color".into()
    }
}
