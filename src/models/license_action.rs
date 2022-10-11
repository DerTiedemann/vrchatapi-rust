/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LicenseAction {
    #[serde(rename = "wear")]
    Wear,
    #[serde(rename = "have")]
    Have,

}

impl ToString for LicenseAction {
    fn to_string(&self) -> String {
        match self {
            Self::Wear => String::from("wear"),
            Self::Have => String::from("have"),
        }
    }
}

impl Default for LicenseAction {
    fn default() -> LicenseAction {
        Self::Wear
    }
}



