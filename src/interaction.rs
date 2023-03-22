use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

use twilight_model::application::interaction::{Interaction, InteractionData, InteractionType};

use crate::command::{init_commands, CommandInput};
use crate::component::{init_components, ComponentInput};
use crate::error::{Error, InteractionError};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};

#[derive(Deserialize, Serialize)]
pub(crate) struct Context {
    pub(crate) interaction: Interaction,
}

impl Context {
    pub(crate) fn handle_ping(&self) -> InteractionResponse {
        InteractionResponse {
            kind: twilight_model::http::interaction::InteractionResponseType::Pong,
            data: None,
        }
    }

    pub(crate) async fn handle_command(
        &self,
        ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionResponse, InteractionError> {
        if let Some(InteractionData::ApplicationCommand(data)) = self.interaction.data.clone() {
            let commands = init_commands();

            let command_input = CommandInput {
                id: data.id,
                name: data.name.clone(),
                resolved: data.resolved,
                kind: data.kind,
                target_id: data.target_id,
                options: data.options,
                guild_id: self.interaction.guild_id,
                channel_id: self.interaction.channel_id,
                user: self.interaction.user.as_ref(),
                member: self.interaction.member.as_ref(),
                ctx: ctx,
            };

            for boxed in commands.iter() {
                let com = boxed;
                if com.name() == data.name {
                    let data = com.respond(&command_input).await?;
                    return Ok(InteractionResponse {
                        data: Some(data),
                        kind: InteractionResponseType::ChannelMessageWithSource,
                    });
                }
            }
            Err(InteractionError::UnknownCommand(data.name))
        } else {
            unreachable!()
        }
    }

    pub(crate) async fn handle_autocomplete(
        &self,
        ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionResponse, InteractionError> {
        if let Some(InteractionData::ApplicationCommand(data)) = self.interaction.data.clone() {
            let commands = init_commands();

            let command_input = CommandInput {
                id: data.id,
                name: data.name.clone(),
                resolved: data.resolved,
                kind: data.kind,
                target_id: data.target_id,
                options: data.options,
                guild_id: self.interaction.guild_id,
                channel_id: self.interaction.channel_id,
                user: self.interaction.user.as_ref(),
                member: self.interaction.member.as_ref(),
                ctx: ctx,
            };

            for boxed in commands.iter() {
                let com = boxed;
                if com.name() == data.name {
                    let data = com.autocomplete(&command_input).await?;
                    return Ok(InteractionResponse {
                        data,
                        kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
                    });
                }
            }
            Err(InteractionError::UnknownCommand(data.name))
        } else {
            unreachable!()
        }
    }

    pub(crate) async fn handle_message_component(
        &self,
        ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionResponse, InteractionError> {
        if let Some(InteractionData::MessageComponent(data)) = self.interaction.data.clone() {
            let components = init_components();

            let component_input = ComponentInput {
                custom_id: data.custom_id.clone(),
                component_type: data.component_type,
                values: data.values,
                guild_id: self.interaction.guild_id,
                channel_id: self.interaction.channel_id,
                user: self.interaction.member.as_ref(),
                member: self.interaction.member.as_ref(),
                message: self.interaction.message.as_ref(),
                ctx: ctx,
            };
            for boxed in components.iter() {
                let com = boxed;
                if data.custom_id.starts_with(&com.custom_id()) {
                    return com.respond(&component_input).await;
                }
            }
            Err(InteractionError::UnknownCommand(data.custom_id))
        } else {
            unreachable!();
        }
    }

    pub(crate) async fn perform(
        &self,
        ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionResponse, Error> {
        match self.interaction.kind {
            InteractionType::Ping => Ok(self.handle_ping()),
            InteractionType::ApplicationCommand => self
                .handle_command(ctx)
                .await
                .map_err(Error::InteractionFailed),
            InteractionType::MessageComponent => self
                .handle_message_component(ctx)
                .await
                .map_err(Error::InteractionFailed),
            InteractionType::ApplicationCommandAutocomplete => self
                .handle_autocomplete(ctx)
                .await
                .map_err(Error::InteractionFailed),
            _ => Err(Error::InvalidPayload("Not implemented".into())),
        }
    }

    pub(crate) fn new(interaction: Interaction) -> Self {
        Self { interaction }
    }
}
