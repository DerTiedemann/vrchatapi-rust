/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// GroupPermission : A permission that can be granted to a role in a group.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupPermission {
    /// The name of the permission.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The display name of the permission.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Human-readable description of the permission.
    #[serde(rename = "help", skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    /// Whether this permission is a \"management\" permission.
    #[serde(rename = "isManagementPermission", skip_serializing_if = "Option::is_none")]
    pub is_management_permission: Option<bool>,
    /// Whether the user is allowed to add this permission to a role.
    #[serde(rename = "allowedToAdd", skip_serializing_if = "Option::is_none")]
    pub allowed_to_add: Option<bool>,
}

impl GroupPermission {
    /// A permission that can be granted to a role in a group.
    pub fn new() -> GroupPermission {
        GroupPermission {
            name: None,
            display_name: None,
            help: None,
            is_management_permission: None,
            allowed_to_add: None,
        }
    }
}


