use schemars::schema::RootSchema;
use serde_json::Value;
use tauri::State;

use crate::state::AppState;

pub struct CommandSpec {
    pub operation_id: String,
    pub request: RootSchema,
    pub response: RootSchema,
    pub error: RootSchema,
}

pub trait Command: Send + Sync {
    fn operation_id(&self) -> String;
    fn run(&self, state: &State<'_, AppState>, request: Value) -> Result<Value, Value>;
    fn command_spec(&self) -> CommandSpec;
}
