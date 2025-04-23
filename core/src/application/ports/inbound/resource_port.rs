use crate::domain::models::{
    resource_groups::{ResourceGroup, ResourceGroupIdentifier},
    resources::{ChildResource, ResourceIdentifier, RootResource},
};

pub trait ResourcePort: Send + Sync {
    fn get_root_resource(&self) -> Result<RootResource, ResourcePortError>;

    fn get_resource_groups(
        &self,
        parent_resource_id: &ResourceIdentifier,
    ) -> Result<GetResourceGroupsResponse, ResourcePortError>;

    fn get_resources_for_group(
        &self,
        group_id: &ResourceGroupIdentifier,
    ) -> Result<GetResourcesResponse, ResourcePortError>;
}

#[derive(Debug)]
pub enum ResourcePortError {}

pub struct GetResourceGroupsResponse {
    pub data: Vec<ResourceGroup>,
}

pub struct GetResourcesResponse {
    pub data: Vec<ChildResource>,
}
