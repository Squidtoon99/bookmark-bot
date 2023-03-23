use async_trait::async_trait;
use twilight_model::channel::{message::{Component, component::{ActionRow, Button, ButtonStyle}, ReactionType}};
#[async_trait(?Send)]
pub(crate) trait SharedInput<'a> {
    fn default_components<S: ToString>(&self, jump_url: S) -> Vec<twilight_model::channel::message::component::Component> {
        vec![Component::ActionRow(ActionRow {
                        components: vec![
                            Component::Button(Button {
                                custom_id: Some("color".to_string()),
                                disabled: false,
                                emoji: Some(ReactionType::Unicode {
                                    name: "🎨".to_string(),
                                }),
                                label: None,
                                style: ButtonStyle::Secondary,
                                url: None,
                            }),
                            Component::Button(Button {
                                custom_id: Some("delete".to_string()),
                                disabled: false,
                                emoji: Some(ReactionType::Unicode {
                                    name: "❌".to_string(),
                                }),
                                label: None,
                                style: ButtonStyle::Secondary,
                                url: None,
                            }),
                            Component::Button(Button {
                                custom_id: None,
                                disabled: false,
                                //  link emoji
                                label: None,
                                style: ButtonStyle::Link,
                                url: Some(jump_url.to_string()),
                                emoji: Some(ReactionType::Unicode {
                                    name: "🔗".to_string(),
                                }),
                            }),
                        ],
                    })]
    }
}