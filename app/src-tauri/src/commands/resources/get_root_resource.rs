use crate::{
    commands::{
        command::{Command, CommandTSModels},
        models::{errors::CommandError, resources::RootResource},
    },
    state::AppState,
};
use serde::Serialize;
use serde_json::Value;
use tauri::State;
use ts_rs::TS;

//
// ─── TYPES ────────────────────────────────────────────────────────────────
//

#[derive(Serialize, TS)]
pub struct GetRootResourceRequest {}

#[derive(Serialize, TS)]
pub struct GetRootResourceResponse {
    resource: RootResource,
}

#[derive(Debug, Serialize, TS)]
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

    fn generate_ts_models(&self) -> CommandTSModels {
        CommandTSModels {
            request: GetRootResourceRequest::decl(),
            response: GetRootResourceResponse::decl(),
            error: GetRootResourceErrors::decl(),
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
