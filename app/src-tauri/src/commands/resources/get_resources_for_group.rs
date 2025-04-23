use crate::{
    commands::{
        command::{Command, CommandSpec},
        models::{
            errors::CommandError,
            resources::{ChildResourceDto, ResourceGroupIdentifierDto},
        },
        utils::serialize,
    },
    state::AppState,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::State;

//
// ─── TYPES ────────────────────────────────────────────────────────────────
//

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetResourcesForGroupRequest {
    group_id: ResourceGroupIdentifierDto,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetResourcesForGroupResponse {
    data: Vec<ChildResourceDto>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(tag = "type", content = "error")]
#[serde(rename_all = "camelCase")]
pub enum GetResourcesForGroupErrors {
    GenericError(CommandError),
}

//
// ─── COMMAND IMPLEMENTATION ──────────────────────────────────────────────
//

pub struct GetResourcesForGroupCmd;

impl Command for GetResourcesForGroupCmd {
    fn operation_id(&self) -> String {
        "__resources_get_resources_for_group".into()
    }

    fn command_spec(&self) -> CommandSpec {
        CommandSpec {
            operation_id: self.operation_id(),
            request: schemars::schema_for!(GetResourcesForGroupRequest),
            response: schemars::schema_for!(GetResourcesForGroupResponse),
            error: schemars::schema_for!(GetResourcesForGroupErrors),
        }
    }

    fn run(&self, state: &State<'_, AppState>, request: Value) -> Result<Value, Value> {
        let resource_port = &state.resource_port;
        let request =
            serde_json::from_value::<GetResourcesForGroupRequest>(request).map_err(|_| {
                serialize(GetResourcesForGroupErrors::GenericError(CommandError::new(
                    "Failed to parse request",
                )))
            })?;

        resource_port
            .get_resources_for_group(&request.group_id.into())
            .map(|res| {
                serialize(GetResourcesForGroupResponse {
                    data: res.data.into_iter().map(ChildResourceDto::from).collect(),
                })
            })
            .map_err(|_| {
                serialize(GetResourcesForGroupErrors::GenericError(CommandError::new(
                    "Failed to get resources for group",
                )))
            })
    }
}
