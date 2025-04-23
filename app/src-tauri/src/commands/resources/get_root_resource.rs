use crate::{
    commands::{
        command::{Command, CommandSpec},
        models::{errors::CommandError, resources::RootResource},
    },
    state::AppState,
};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;
use tauri::State;

//
// ─── TYPES ────────────────────────────────────────────────────────────────
//

#[derive(Serialize, JsonSchema)]
pub struct GetRootResourceRequest {}

#[derive(Serialize, JsonSchema)]
pub struct GetRootResourceResponse {
    resource: RootResource,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(tag = "type", content = "error")]
pub enum GetRootResourceErrors {
    GenericError(CommandError),
}

//
// ─── COMMAND IMPLEMENTATION ──────────────────────────────────────────────
//

pub struct GetRootResourceCmd;

fn serialize<T: Serialize>(v: T) -> Value {
    serde_json::to_value(v).unwrap()
}

impl Command for GetRootResourceCmd {
    fn operation_id(&self) -> String {
        "__resources_get_root_resource".into()
    }

    fn command_spec(&self) -> CommandSpec {
        CommandSpec {
            operation_id: self.operation_id(),
            request: schemars::schema_for!(GetRootResourceRequest),
            response: schemars::schema_for!(GetRootResourceResponse),
            error: schemars::schema_for!(GetRootResourceErrors),
        }
    }

    fn run(&self, state: &State<'_, AppState>, _request: Value) -> Result<Value, Value> {
        let resource_port = &state.resource_port;

        resource_port
            .get_root_resource()
            .map(|res| {
                serialize(GetRootResourceResponse {
                    resource: res.into(),
                })
            })
            .map_err(|_| {
                serialize(GetRootResourceErrors::GenericError(CommandError::new(
                    "Failed to get root resource",
                )))
            })
    }
}
