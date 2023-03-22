use reqwest::{ClientBuilder, header::{CONTENT_TYPE, AUTHORIZATION, HeaderMap}, Client};
use serde::{Deserialize, Serialize};

use twilight_model::{
    application::{
        command::{CommandOption, CommandType},
        interaction::application_command::{CommandDataOption, CommandInteractionDataResolved, CommandOptionValue},
    },
    id::{
        marker::{CommandMarker, GenericMarker, GuildMarker, ChannelMarker, UserMarker},
        Id,
    }, guild::PartialMember, user::User
};

use crate::{commands, input::SharedInput};
use crate::error::InteractionError;

use async_trait::async_trait;
use twilight_model::http::interaction::InteractionResponseData;
#[allow(dead_code)]
pub(crate) struct CommandInput<'a> {
    pub(crate) channel_id: Option<Id<ChannelMarker>>,
    pub(crate) user: Option<&'a twilight_model::user::User>,
    pub(crate) member: Option<&'a PartialMember>,
    pub(crate) ctx: &'a mut worker::RouteContext<()>,

    pub(crate) guild_id: Option<Id<GuildMarker>>,
    pub(crate) id: Id<CommandMarker>,
    pub name: String,
    pub kind: CommandType,
    pub options: Vec<CommandDataOption>,
    pub resolved: Option<CommandInteractionDataResolved>,
    pub target_id: Option<Id<GenericMarker>>,
}

impl SharedInput<'_> for CommandInput<'_> {}

#[allow(dead_code)]
impl CommandInput<'_> {
    pub fn get_option(&self, name: &str) -> Option<CommandOptionValue> {
        for option in &self.options {
            if option.name == name {
                return Some(option.value.clone());
            }
        }
        None
    }

    pub async fn kv_get(
        &self,
        namespace: &str,
        key: &str,
    ) -> Result<Option<String>, InteractionError> {
        let kv = self
            .ctx
            .kv(namespace)
            .map_err(|_| InteractionError::WorkerError("Bind to kv".into()))?;
        let value = kv
            .get(key)
            .text()
            .await
            .map_err(|_| InteractionError::WorkerError("Fetching from KV".into()))?;
        Ok(value)
    }

    pub async fn kv_put(
        &self,
        namespace: &str,
        key: &str,
        value: &str,
    ) -> Result<(), InteractionError> {
        let kv = self
            .ctx
            .kv(namespace)
            .map_err(|_| InteractionError::WorkerError("bind to kv".into()))?;
        kv.put(key, value)
            .map_err(|_| InteractionError::WorkerError("bind to KV".into()))?
            .execute()
            .await
            .map_err(|_| InteractionError::WorkerError("KV put".into()))?;
        Ok(())
    }

    pub(crate) fn uid(&self) -> Result<Id<UserMarker>, InteractionError> {
        if let Some(u) = self.member.as_ref().and_then(|m| m.user.as_ref()) {
            Ok(u.id)
        } else if let Some(u) = self.user.as_ref() {
            Ok(u.id)
        } else {
            Err(InteractionError::WorkerError("No member".into()))
        }
    }

    pub(crate) fn http_client(&self) -> Result<Client, InteractionError> {
        let token = self.ctx.var("DISCORD_TOKEN")?.to_string();
        
        let mut headers = HeaderMap::new();

        headers.append(
            AUTHORIZATION,
            format!("Bot {}",token)
                .parse()
                .unwrap(),
        );

        headers.append(CONTENT_TYPE, "application/json".parse().unwrap());
        Ok(ClientBuilder::new()
            .default_headers(headers)
            .build()?)
    }
}

#[async_trait(?Send)]
pub(crate) trait Command {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionResponseData, InteractionError>;

    fn name(&self) -> String; // The command name, ie `return "greet".to_string()` for /greet
  

    fn description(&self) -> String {
        // A short description
        "".into()
    }

    fn options(&self) -> Option<Vec<CommandOption>> {
        // add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
        None
    }

    fn kind(&self) -> CommandType {
        CommandType::ChatInput
    }

    async fn autocomplete(
        &self,
        _input: &CommandInput,
    ) -> Result<Option<InteractionResponseData>, InteractionError> {
        // If your command supports autocomplete implement the logic here
        Ok(None)
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RegisteredCommand {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) options: Option<Vec<CommandOption>>,
    #[serde(rename = "type")]
    pub(crate) kind: CommandType,
}

pub(crate) fn init_commands() -> Vec<Box<dyn Command + Sync>> {
    let mut v: Vec<Box<dyn Command + Sync>> = Vec::new();
    v.push(Box::new(commands::help::Help {}));
    v.push(Box::new(commands::bookmark::Bookmark {}));
    v
}
