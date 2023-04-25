use crate::command::{Command, CommandInput};
use crate::error::InteractionError;

use async_trait::async_trait;
use serde_json::json;
use twilight_model::application::command::CommandOption;
use twilight_model::channel::message::component::{ButtonStyle, Button, ActionRow};
use twilight_model::channel::message::{Embed, MessageFlags, Component};
use twilight_model::http::interaction::InteractionResponseData;
use twilight_util::builder::InteractionResponseDataBuilder;

pub(crate) struct Help {}

#[async_trait(?Send)]
impl Command for Help {
    async fn respond(
        &self,
        input: &CommandInput,
    ) -> Result<InteractionResponseData, InteractionError> {

        
        // return a different message if it's being sent in dms or not
        let guild_help = serde_json::from_value(json!({
              "title": "Server Help",
              "type": "rich",
              "description": "Bookermarker is a simple bot that allows users to bookmark messages by using interactions. Right click on a message --> Apps --> Bookmark. The bot will DM you with the contents of the message.",
              "color": 3092790,
              "fields": [
                {
                  "name": "Command Permissions",
                  "value": "To manage in which roles / channels Bookmarker can be used, head to Server settings --> Integrations --> Bookmarker and adjust the **Bookmark** command. For more information on managing slash command perms see this [discord article.](https://support.discord.com/hc/en-us/articles/10952896421783)"
                }
              ],
              "image": {
                "url": "https://i.imgur.com/g5nLSDR.png"
              }
            }))?;
        
        let dm_help = serde_json::from_value(json!({
              "title": "DM Help",
              "description": "Bookmarker offers some unique interactions in DMs to help organise and add notes to bookmarks.",
              "type": "rich",
              "color": 3092790,
              "fields": [
                {
                  "name": ":pencil: Add Note (coming soon)",
                  "value": "Adds a note to the bookmark. Leave blank to remove the note",
                  "inline": true
                },
                {
                  "name": ":art: Change Embed Colour",
                  "value": "Update the colour of the embed from a selection.",
                  "inline": true
                },
                {
                  "name": ":x: Delete Bookmark",
                  "value": "Deletes the bookmark instantly.",
                  "inline": true
                }
              ]
            }))?;
        
        let components = Component::ActionRow(ActionRow {
            components: vec![
                // invite bot button
                Component::Button(Button {
                    style: ButtonStyle::Link,
                    label: Some("Invite".into()),
                    emoji: None,
                    custom_id: None,
                    url: Some(format!("https://discord.com/api/oauth2/authorize?client_id={}&permissions=0&scope=bot%20applications.commands", &input.ctx.var("DISCORD_APPLICATION_ID").expect("DISCORD_APPLICATION_ID not set").to_string())),
                    disabled: false,
                }),
                // support form button
                Component::Button(Button {
                    style: ButtonStyle::Link,
                    label: Some("Support Form".into()),
                    emoji: None,
                    custom_id: None,
                    url: Some("https://tally.so/r/w7q1EA/".into()),
                    disabled: false,
                }),
            ]
        });

        let embeds: Vec<Embed>;
        if input.guild_id.is_none() {
            embeds = vec![dm_help];
        } else {
            embeds = vec![guild_help, dm_help];
        }

        Ok(InteractionResponseDataBuilder::new()
            .flags(MessageFlags::EPHEMERAL)
            .embeds(embeds)
            .components([components])
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
