/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LimitedUser {
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "developerType")]
    pub developer_type: crate::models::DeveloperType,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fallbackAvatar")]
    pub fallback_avatar: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "last_platform")]
    pub last_platform: String,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    /// <- Always empty.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "friendKey", skip_serializing_if = "Option::is_none")]
    pub friend_key: Option<String>,
}

impl LimitedUser {
    pub fn new(current_avatar_image_url: String, current_avatar_thumbnail_image_url: String, developer_type: crate::models::DeveloperType, display_name: String, fallback_avatar: String, id: String, is_friend: bool, last_platform: String, profile_pic_override: String, status: crate::models::UserStatus, status_description: String, tags: Vec<String>, user_icon: String, username: String) -> LimitedUser {
        LimitedUser {
            bio: None,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
            developer_type,
            display_name,
            fallback_avatar,
            id,
            is_friend,
            last_platform,
            profile_pic_override,
            status,
            status_description,
            tags,
            user_icon,
            username,
            location: None,
            friend_key: None,
        }
    }
}


