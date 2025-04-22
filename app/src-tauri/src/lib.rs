pub mod commands;
pub mod state;

pub mod generated;

use commands::{
    gateway::cmd_gateway, registry::CommandRegistry,
    resources::get_root_resource::GetRootResourceCmd,
};
use mydatamess_core::application::services::resource_service::ResourceService;
use state::AppState;

pub fn build_cmd_registry() -> CommandRegistry {
    CommandRegistry::new().add_cmd(GetRootResourceCmd)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        resource_port: Box::new(ResourceService::new()),
        cmd_registry: build_cmd_registry(),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![cmd_gateway])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
