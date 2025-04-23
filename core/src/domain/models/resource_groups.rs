use super::resources::ResourceIdentifier;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ResourceGroupIdentifier {
    pub parent_resource_id: ResourceIdentifier,
    pub key: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ResourceGroup {
    pub id: ResourceGroupIdentifier,
    pub display_name: String,
    pub description: Option<String>,
    pub priority: Option<u16>,
}
