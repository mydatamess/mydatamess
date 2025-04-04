use tauri::State;

use crate::{commands::models::resources::RootResource, state::AppState};

use super::models::errors::CommandError;

#[tauri::command]
pub async fn __resources_get_root_resource(
    state: State<'_, AppState>,
) -> Result<RootResource, get_root_resource::Errors> {
    use get_root_resource::*;
    let resource_port = &state.resource_port;

    Err(Errors::GenericError(CommandError::new(
        "Failed to get root resource",
    )))

    // resource_port
    //     .get_root_resource()
    //     .map(RootResource::from)
    //     .map_err(|_| Errors::ServerError(CommandError::new("Failed to get root resource")))
}

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

mod get_root_resource {
    use crate::commands::models::errors::CommandError;

    #[derive(Debug, serde::Serialize)]
    #[serde(tag = "type", content = "error")]
    pub enum Errors {
        GenericError(CommandError),
    }
}
