use super::resource_groups::ResourceGroupIdentifier;

pub type ResourceIdentifier = String;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ResourceBase {
    pub id: ResourceIdentifier,
    pub display_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RootResource {
    pub base: ResourceBase,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ChildResource {
    pub base: ResourceBase,
    pub group_id: ResourceGroupIdentifier,
}

pub enum Resource {
    Root(RootResource),
    Child(ChildResource),
}
