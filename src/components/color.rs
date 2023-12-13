use crate::component::{Component as ComponentTrait, ComponentInput};
use crate::error::InteractionError;
use crate::input::SharedInput;
use crate::utils::bookmark_data;
use twilight_model::channel::Message;
use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle},
    Component,
};

use async_trait::async_trait;
use twilight_model::channel::message::{ReactionType, MessageFlags, Embed};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::Id;
use twilight_util::builder::InteractionResponseDataBuilder;
use twilight_util::builder::embed::EmbedBuilder;
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
            let (action, url) = data.split_once(":").unwrap();
            let actual_url = format!("https://discord.com/channels/{}", &url);
            console_log!("action: {}, url: {}", action, url);
            let mut embeds: Vec<Embed>;
            let mut kind = InteractionResponseType::UpdateMessage;
            match action {
                "update" => {
                    // fetch new bookmark data
                    let client = input.http_client()?;
                    let mut parts = url.splitn(3, "/");
                    let (guild_id, channel_id, message_id) = (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap());
                    let url = format!("https://discord.com/api/v10/channels/{}/messages/{}", channel_id, message_id);
                    if let Ok(response) = client.get(url).send().await {
                        let text  =response.text().await?;
                        let msg_data: Message = serde_json::from_str(&text)?;
                        embeds = bookmark_data(&msg_data, &client, guild_id).await?
                    } else {
                        return Ok(InteractionResponse {
                            data: Some(
                                InteractionResponseDataBuilder::new()
                                    .embeds(vec![
                                        EmbedBuilder::new()
                                            .title("Message Edit Error")
                                            .description("```\nI can no longer has access to the original message. It may have been deleted or hidden from me\n```")
                                            .build()
                                    ])
                                    .flags(MessageFlags::EPHEMERAL)
                                    .build()
                                ),
                            kind: InteractionResponseType::ChannelMessageWithSource
                        })
                    }
                },
                "repost" => {   
                    let client = input.http_client()?;
                    let (guild_id, _extra) = url.split_once("/").unwrap();
                    
                    embeds = bookmark_data(&input.message.unwrap(), &client, guild_id).await?;
                    kind = InteractionResponseType::ChannelMessageWithSource;
                },
                color  => {
                    // change the bookmark embeds color
                    let msg = input.message.unwrap();
                    embeds = msg.embeds.clone();

                    for mut embed in embeds.iter_mut() {
                        embed.color = Some(color.parse::<u32>().unwrap());
                    }
                    
                }
            }

            Ok(InteractionResponse {
                kind: kind,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .embeds(embeds)
                        .components(input.default_components(actual_url))
                        .content("")
                        .build(),
                ),
            })
        } else {
            let Some(Component::ActionRow(row)) = input.message.unwrap().components.first() else {
                    return Err(InteractionError::WorkerError("No components found".to_string()))
                };
            let Some(Component::Button(Button {url, ..})) = row.components.last() else {
                    return Err(InteractionError::WorkerError("No components found".to_string()))
                };
            let url = {
                let u = url.as_ref().unwrap();
                let (_, data) = u.split_at(29);
                data.to_string()
            };
            let color_components = Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: None,
                        custom_id: Some(format!("color:5793266:{url}", url = url)),
                        emoji: Some(ReactionType::Custom {
                            id: Id::new(1184416010254684201),
                            name: Some("bk_blurple".to_string()),
                            animated: false
                        }),
                        url: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: None,
                        custom_id: Some(format!("color:15548997:{url}", url = url)),
                        // <:bk_red:1086977702386466867>
                        emoji: Some(ReactionType::Custom {
                            id: Id::new(1184416103464714300),
                            name: Some("bk_red".to_string()),
                            animated: false
                        }),
                        url: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: None,
                        custom_id: Some(format!("color:5763719:{url}", url = url)),
                        // <:bk_green:1086977699488223384>
                        emoji: Some(ReactionType::Custom {
                            id: Id::new(1184416108351070250),
                            name: Some("bk_green".to_string()),
                            animated: false
                        }),
                        url: None,
                        disabled: false,
                    }),
                    // <:bk_yellow:1086977704194216066>
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: None,
                        custom_id: Some(format!("color:16705372:{url}", url = url)),
                        emoji: Some(ReactionType::Custom {
                            id: Id::new(1184416111316435045),
                            name: Some("bk_yellow".to_string()),
                            animated: false
                        }),
                        url: None,
                        disabled: false,
                    }),
                    // <:bk_fucahsia:1086977696216645657>
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        label: None,
                        custom_id: Some(format!("color:15418782:{url}", url = url)),
                        emoji: Some(ReactionType::Custom {
                            id: Id::new(1184416105251471480),
                            name: Some("bk_fucahsia".to_string()),
                            animated: false
                        }),
                        url: None,
                        disabled: false,
                    }),
                ],
            });
            
            let action_components = Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        style: ButtonStyle::Primary,
                        label: Some("Refresh Data".to_string()),
                        custom_id: Some(format!("color:update:{url}", url = url)),
                        emoji: Some(ReactionType::Unicode {
                            name: "🔃".to_string()
                        }),
                        url: None,
                        disabled: false
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Primary,
                        label: Some("Repost Bookmark".to_string()),
                        custom_id: Some(format!("color:repost:{url}", url = url)),
                        emoji: Some(ReactionType::Unicode {
                            name: "📑".to_string()
                        }),
                        url: None,
                        disabled: false
                    }),
                ]
            });
            let data = InteractionResponseDataBuilder::new()
                .components([color_components, action_components])
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
