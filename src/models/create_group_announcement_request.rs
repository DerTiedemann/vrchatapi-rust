/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateGroupAnnouncementRequest {
    /// Announcement title
    #[serde(rename = "title")]
    pub title: String,
    /// Announcement text
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Send notification to group members.
    #[serde(rename = "sendNotification", skip_serializing_if = "Option::is_none")]
    pub send_notification: Option<bool>,
}

impl CreateGroupAnnouncementRequest {
    pub fn new(title: String) -> CreateGroupAnnouncementRequest {
        CreateGroupAnnouncementRequest {
            title,
            text: None,
            image_id: None,
            send_notification: None,
        }
    }
}


