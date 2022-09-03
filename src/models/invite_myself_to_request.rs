/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteMyselfToRequest {
    /// Short Name of the Instance; can be retrieved from the Get Instance Short Name endpoint.
    #[serde(rename = "shortName")]
    pub short_name: String,
}

impl InviteMyselfToRequest {
    pub fn new(short_name: String) -> InviteMyselfToRequest {
        InviteMyselfToRequest {
            short_name,
        }
    }
}


