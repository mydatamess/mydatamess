use mydatamess_core::domain::models::resources::RootResource as CoreRootResource;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootResource {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
}

impl From<CoreRootResource> for RootResource {
    fn from(value: CoreRootResource) -> Self {
        RootResource {
            id: value.base.id,
            display_name: value.base.display_name,
            description: value.base.description,
        }
    }
}
