mod commands;
mod state;

use commands as cmds;
use mydatamess_core::application::services::resource_service::ResourceService;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        resource_port: Box::new(ResourceService::new()),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            cmds::resources::__resources_get_resources
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
