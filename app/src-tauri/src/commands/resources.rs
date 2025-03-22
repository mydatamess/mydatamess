use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub async fn __resources_get_resources(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let resource_port = &state.resource_port;
    let root_resource = resource_port.get_root_resource().unwrap();
    let binding = resource_port
        .get_resource_groups(&root_resource.base.id)
        .unwrap();
    let resource_group = binding.data.first().unwrap();

    let res = resource_port
        .get_resources_for_group(&resource_group.id)
        .unwrap()
        .data
        .iter()
        .map(|resource| resource.base.display_name.clone())
        .collect();

    Ok(res)
}
