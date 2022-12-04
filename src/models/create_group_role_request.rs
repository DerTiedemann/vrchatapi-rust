/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateGroupRoleRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isSelfAssignable", skip_serializing_if = "Option::is_none")]
    pub is_self_assignable: Option<bool>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl CreateGroupRoleRequest {
    pub fn new() -> CreateGroupRoleRequest {
        CreateGroupRoleRequest {
            id: None,
            name: None,
            description: None,
            is_self_assignable: None,
            permissions: None,
        }
    }
}


