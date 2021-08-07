/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://raw.githubusercontent.com/vrchatapi/vrchatapi.github.io/master/assets/apibanner.png)  # VRChat API Documentation This project is an [OPEN Open Source Project](https://openopensource.org)  Individuals making significant and valuable contributions are given commit-access to the project to contribute as they see fit. This project is more like an open wiki than a standard guarded open source project.  ## Disclaimer  This is the official response of the VRChat Team (from Tupper more specifically) on the usage of the VRChat API.  > **Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:** > * We do not provide documentation or support for the API. > * Do not make queries to the API more than once per 60 seconds. > * Abuse of the API may result in account termination. > * Access to API endpoints may break at any given time, with no warning.  As stated, this documentation was not created with the help of the official VRChat team. Therefore this documentation is not an official documentation of the VRChat API and may not be always up to date with the latest versions. If you find that a page or endpoint is not longer valid please create an issue and tell us so we can fix it.  ## Get in touch with us!  [https://discord.gg/qjZE9C9fkB#vrchat-api](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "pastDisplayNames")]
    pub past_display_names: Vec<String>,
    #[serde(rename = "hasEmail")]
    pub has_email: bool,
    #[serde(rename = "hasPendingEmail")]
    pub has_pending_email: bool,
    #[serde(rename = "obfuscatedEmail")]
    pub obfuscated_email: String,
    #[serde(rename = "obfuscatedPendingEmail")]
    pub obfuscated_pending_email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "hasBirthday")]
    pub has_birthday: bool,
    #[serde(rename = "unsubscribe")]
    pub unsubscribe: bool,
    #[serde(rename = "statusHistory")]
    pub status_history: Vec<String>,
    #[serde(rename = "statusFirstTime")]
    pub status_first_time: bool,
    #[serde(rename = "friends")]
    pub friends: Vec<String>,
    #[serde(rename = "friendGroupNames")]
    pub friend_group_names: Vec<String>,
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "fallbackAvatar")]
    pub fallback_avatar: String,
    #[serde(rename = "currentAvatar")]
    pub current_avatar: String,
    #[serde(rename = "currentAvatarAssetUrl")]
    pub current_avatar_asset_url: String,
    #[serde(rename = "accountDeletionDate", skip_serializing_if = "Option::is_none")]
    pub account_deletion_date: Option<String>,
    #[serde(rename = "acceptedTOSVersion")]
    pub accepted_tos_version: f32,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    #[serde(rename = "steamDetails")]
    pub steam_details: serde_json::Value,
    #[serde(rename = "oculusId")]
    pub oculus_id: String,
    #[serde(rename = "hasLoggedInFromClient")]
    pub has_logged_in_from_client: bool,
    #[serde(rename = "homeLocation")]
    pub home_location: String,
    #[serde(rename = "twoFactorAuthEnabled")]
    pub two_factor_auth_enabled: bool,
    #[serde(rename = "state")]
    pub state: crate::models::UserState,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "developerType")]
    pub developer_type: crate::models::DeveloperType,
    #[serde(rename = "last_login")]
    pub last_login: String,
    #[serde(rename = "last_platform")]
    pub last_platform: crate::models::Platform,
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "date_joined")]
    pub date_joined: String,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    #[serde(rename = "friendKey")]
    pub friend_key: String,
    #[serde(rename = "onlineFriends")]
    pub online_friends: Vec<String>,
    #[serde(rename = "activeFriends")]
    pub active_friends: Vec<String>,
    #[serde(rename = "offlineFriends")]
    pub offline_friends: Vec<String>,
}

impl CurrentUser {
    pub fn new(id: String, username: String, display_name: String, user_icon: String, bio: String, bio_links: Vec<String>, profile_pic_override: String, status_description: String, past_display_names: Vec<String>, has_email: bool, has_pending_email: bool, obfuscated_email: String, obfuscated_pending_email: String, email_verified: bool, has_birthday: bool, unsubscribe: bool, status_history: Vec<String>, status_first_time: bool, friends: Vec<String>, friend_group_names: Vec<String>, current_avatar_image_url: String, current_avatar_thumbnail_image_url: String, fallback_avatar: String, current_avatar: String, current_avatar_asset_url: String, accepted_tos_version: f32, steam_id: String, steam_details: serde_json::Value, oculus_id: String, has_logged_in_from_client: bool, home_location: String, two_factor_auth_enabled: bool, state: crate::models::UserState, tags: Vec<String>, developer_type: crate::models::DeveloperType, last_login: String, last_platform: crate::models::Platform, allow_avatar_copying: bool, status: crate::models::UserStatus, date_joined: String, is_friend: bool, friend_key: String, online_friends: Vec<String>, active_friends: Vec<String>, offline_friends: Vec<String>) -> CurrentUser {
        CurrentUser {
            id,
            username,
            display_name,
            user_icon,
            bio,
            bio_links,
            profile_pic_override,
            status_description,
            past_display_names,
            has_email,
            has_pending_email,
            obfuscated_email,
            obfuscated_pending_email,
            email_verified,
            has_birthday,
            unsubscribe,
            status_history,
            status_first_time,
            friends,
            friend_group_names,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
            fallback_avatar,
            current_avatar,
            current_avatar_asset_url,
            account_deletion_date: None,
            accepted_tos_version,
            steam_id,
            steam_details,
            oculus_id,
            has_logged_in_from_client,
            home_location,
            two_factor_auth_enabled,
            state,
            tags,
            developer_type,
            last_login,
            last_platform,
            allow_avatar_copying,
            status,
            date_joined,
            is_friend,
            friend_key,
            online_friends,
            active_friends,
            offline_friends,
        }
    }
}


