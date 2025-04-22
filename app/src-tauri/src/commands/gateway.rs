use crate::state::AppState;
use serde_json::Value;
use tauri::State;

use super::models::errors::CommandError;

#[tauri::command]
pub async fn cmd_gateway(
    state: State<'_, AppState>,
    operation_id: String,
    payload: Value,
) -> Result<Value, Value> {
    let registry = &state.cmd_registry;
    match registry.get(&operation_id) {
        Some(cmd) => cmd.run(&state, payload),
        None => Err(serde_json::to_value(CommandError::new("Operation not found")).unwrap()),
    }
}
