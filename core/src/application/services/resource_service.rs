use uuid::Uuid;

use crate::application::ports::inbound::resource_port::{
    GetResourceGroupsResponse, GetResourcesResponse, ResourcePort, ResourcePortError,
};
use crate::domain::models::{
    resource_groups::{ResourceGroup, ResourceGroupIdentifier},
    resources::{ChildResource, ResourceBase, ResourceIdentifier, RootResource},
};

pub struct ResourceService {}

impl ResourceService {
    pub fn new() -> ResourceService {
        ResourceService {}
    }
}

impl ResourcePort for ResourceService {
    fn get_root_resource(&self) -> Result<RootResource, ResourcePortError> {
        Ok(RootResource {
            base: ResourceBase {
                id: Uuid::new_v4().to_string(),
                display_name: "Home".to_string(),
                description: None,
            },
        })
    }

    fn get_resource_groups(
        &self,
        parent_resource_id: &ResourceIdentifier,
    ) -> Result<GetResourceGroupsResponse, ResourcePortError> {
        Ok(GetResourceGroupsResponse {
            data: vec![ResourceGroup {
                id: ResourceGroupIdentifier {
                    parent_resource_id: parent_resource_id.clone(),
                    key: Uuid::new_v4().to_string(),
                },
                display_name: "Workspaces".to_string(),
                description: None,
                priority: None,
            }],
        })
    }

    fn get_resources_for_group(
        &self,
        group_id: &ResourceGroupIdentifier,
    ) -> Result<GetResourcesResponse, ResourcePortError> {
        Ok(GetResourcesResponse {
            data: vec![
                ChildResource {
                    base: ResourceBase {
                        id: Uuid::new_v4().to_string(),
                        display_name: "Job".to_string(),
                        description: None,
                    },
                    group_id: group_id.clone(),
                },
                ChildResource {
                    base: ResourceBase {
                        id: Uuid::new_v4().to_string(),
                        display_name: "Personal".to_string(),
                        description: None,
                    },
                    group_id: group_id.clone(),
                },
                ChildResource {
                    base: ResourceBase {
                        id: Uuid::new_v4().to_string(),
                        display_name: "Family".to_string(),
                        description: None,
                    },
                    group_id: group_id.clone(),
                },
            ],
        })
    }
}
