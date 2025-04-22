use serde_json::Value;
use tauri::State;
use ts_rs::TS;

use crate::state::AppState;

pub struct CommandTSModels {
    pub request: String,
    pub response: String,
    pub error: String,
}

pub trait Command: Send + Sync {
    fn operation_id(&self) -> String;
    fn run(&self, state: &State<'_, AppState>, request: Value) -> Result<Value, Value>;
    fn generate_ts_models(&self) -> CommandTSModels;
}
