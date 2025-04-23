use crate::{
    commands::{
        command::{Command, CommandSpec},
        models::{errors::CommandError, resources::ResourceGroupDto},
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
pub struct GetResourceGroupsRequest {
    parent_resource_id: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceGroupsResponse {
    data: Vec<ResourceGroupDto>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(tag = "type", content = "error")]
#[serde(rename_all = "camelCase")]
pub enum GetResourceGroupsErrors {
    GenericError(CommandError),
}

//
// ─── COMMAND IMPLEMENTATION ──────────────────────────────────────────────
//

pub struct GetResourceGroupsCmd;

fn serialize<T: Serialize>(v: T) -> Value {
    serde_json::to_value(v).unwrap()
}

impl Command for GetResourceGroupsCmd {
    fn operation_id(&self) -> String {
        "__resources_get_resource_groups".into()
    }

    fn command_spec(&self) -> CommandSpec {
        CommandSpec {
            operation_id: self.operation_id(),
            request: schemars::schema_for!(GetResourceGroupsRequest),
            response: schemars::schema_for!(GetResourceGroupsResponse),
            error: schemars::schema_for!(GetResourceGroupsErrors),
        }
    }

    fn run(&self, state: &State<'_, AppState>, request: Value) -> Result<Value, Value> {
        let resource_port = &state.resource_port;
        let request =
            serde_json::from_value::<GetResourceGroupsRequest>(request).map_err(|_| {
                serialize(GetResourceGroupsErrors::GenericError(CommandError::new(
                    "Failed to parse request",
                )))
            })?;

        resource_port
            .get_resource_groups(&request.parent_resource_id)
            .map(|res| {
                serialize(GetResourceGroupsResponse {
                    data: res.data.into_iter().map(ResourceGroupDto::from).collect(),
                })
            })
            .map_err(|_| {
                serialize(GetResourceGroupsErrors::GenericError(CommandError::new(
                    "Failed to get resource groups",
                )))
            })
    }
}
