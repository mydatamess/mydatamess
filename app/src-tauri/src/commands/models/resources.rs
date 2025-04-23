use mydatamess_core::domain::models::{
    resource_groups::{ResourceGroup, ResourceGroupIdentifier},
    resources::{ChildResource, ResourceBase, RootResource},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceBaseDto {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
}

impl From<ResourceBase> for ResourceBaseDto {
    fn from(value: ResourceBase) -> Self {
        ResourceBaseDto {
            id: value.id,
            display_name: value.display_name,
            description: value.description,
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RootResourceDto {
    #[serde(flatten)]
    pub base: ResourceBaseDto,
}

impl From<RootResource> for RootResourceDto {
    fn from(value: RootResource) -> Self {
        RootResourceDto {
            base: value.base.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGroupIdentifierDto {
    pub parent_resource_id: String,
    pub key: String,
}

impl From<ResourceGroupIdentifier> for ResourceGroupIdentifierDto {
    fn from(value: ResourceGroupIdentifier) -> Self {
        ResourceGroupIdentifierDto {
            parent_resource_id: value.parent_resource_id,
            key: value.key,
        }
    }
}

impl From<ResourceGroupIdentifierDto> for ResourceGroupIdentifier {
    fn from(value: ResourceGroupIdentifierDto) -> Self {
        ResourceGroupIdentifier {
            parent_resource_id: value.parent_resource_id,
            key: value.key,
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGroupDto {
    pub id: ResourceGroupIdentifierDto,
    pub display_name: String,
    pub description: Option<String>,
    pub priority: Option<u16>,
}

impl From<ResourceGroup> for ResourceGroupDto {
    fn from(value: ResourceGroup) -> Self {
        ResourceGroupDto {
            id: value.id.into(),
            display_name: value.display_name,
            description: value.description,
            priority: value.priority,
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChildResourceDto {
    #[serde(flatten)]
    pub base: ResourceBaseDto,
    pub group_id: ResourceGroupIdentifierDto,
}

impl From<ChildResource> for ChildResourceDto {
    fn from(value: ChildResource) -> Self {
        ChildResourceDto {
            base: value.base.into(),
            group_id: value.group_id.into(),
        }
    }
}
