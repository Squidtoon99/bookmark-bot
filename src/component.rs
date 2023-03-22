use twilight_model::{
    channel::{message::component::ComponentType, Message},
    guild::PartialMember,
    http::interaction::InteractionResponse,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

use reqwest::{Client, ClientBuilder, header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE}};
use crate::components;
use crate::input::SharedInput;
use crate::error::InteractionError;
use async_trait::async_trait;

#[allow(dead_code)]
pub(crate) struct ComponentInput<'a> {
    pub(crate) guild_id: Option<Id<GuildMarker>>,
    pub(crate) channel_id: Option<Id<ChannelMarker>>,
    pub(crate) user: Option<&'a PartialMember>,
    pub(crate) member: Option<&'a PartialMember>,
    pub(crate) ctx: &'a mut worker::RouteContext<()>,

    pub(crate) message: Option<&'a Message>,
    pub(crate) custom_id: String,
    pub(crate) component_type: ComponentType,
    pub(crate) values: Vec<String>,
}


impl SharedInput<'_> for ComponentInput<'_> {}

#[allow(dead_code)]
impl ComponentInput<'_> {
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
pub(crate) trait Component {
    async fn respond(
        &self,
        _input: &ComponentInput,
    ) -> Result<InteractionResponse, InteractionError> {
        // Implement the command logic here
        unimplemented!()
    }

    fn custom_id(&self) -> String {
        // The command name, ie `return "greet".to_string()` for /greet
        unimplemented!()
    }
}

pub(crate) fn init_components() -> Vec<Box<dyn Component + Sync>> {
    let mut v: Vec<Box<dyn Component + Sync>> = Vec::new();
    v.push(Box::new(components::delete::Delete {}));
    v.push(Box::new(components::color::Color {}));
    v
}
