use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName", default)]
    pub last_name: Option<String>,
    #[serde(rename = "type")]
    pub name_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub owner: i64,
    #[serde(rename = "hasBots", default)]
    pub has_bots: bool,
    #[serde(rename = "joinTime")]
    pub join_time: i64,
    pub created: i64,
    #[serde(rename = "lastMessage")]
    pub last_message: LastMessage,
    #[serde(rename = "type")]
    pub chat_type: String,
    #[serde(rename = "lastFireDelayedErrorTime")]
    pub last_fire_delayed_error_time: i64,
    #[serde(rename = "lastDelayedUpdateTime")]
    pub last_delayed_update_time: i64,
    pub modified: i64,
    #[serde(rename = "lastEventTime")]
    pub last_event_time: i64,
    pub id: i64,
    pub status: String,
    pub participants: HashMap<String, i64>,
    pub cid: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub options: Option<ChatOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastMessage {
    pub sender: i64,
    //#[serde(default)]
    //pub elements: Option<Vec<MessageElement>>,
    #[serde(default)]
    pub options: Option<i32>,
    pub id: String,
    pub time: i64,
    pub text: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(default)]
    pub attaches: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    #[serde(default)]
    pub preview: Option<Preview>,

    #[serde(default)]
    pub size: Option<i64>,

    #[serde(rename = "_type")]
    pub attachment_type: String,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "fileId", default)]
    pub file_id: Option<i64>,

    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preview {
    #[serde(rename = "previewData")]
    pub preview_data: String,

    #[serde(default)]
    pub duration: Option<i64>,

    #[serde(default)]
    pub thumbnail: Option<String>,

    #[serde(rename = "_type")]
    pub preview_type: String,

    pub width: i32,

    #[serde(rename = "videoId")]
    #[serde(default)]
    pub video_id: Option<i64>,

    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "accountStatus")]
    pub account_status: i32,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub names: Vec<Name>,
    pub phone: i64,
    pub options: Vec<String>,
    #[serde(rename = "photoId")]
    pub photo_id: i64,
    pub description: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    pub id: i64,
    #[serde(rename = "baseRawUrl")]
    pub base_raw_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "accountStatus")]
    pub account_status: i32,
    pub names: Vec<Name>,
    pub gender: i32,
    pub options: Vec<String>,
    pub link: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatsResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: ChatsResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatsResponsePayload {
    Success(ChatsPayloadSuccess),
    Error(ErrorPayload),
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ChatsPayloadSuccess {
    pub profile: Profile,
    pub drafts: Drafts,
    pub token: String,
    #[serde(rename = "videoChatHistory")]
    pub video_chat_history: bool,
    pub calls: Vec<String>,
    pub chats: Vec<Chat>,
    #[serde(rename = "chatMarker")]
    pub chat_marker: i64,
    pub messages: HashMap<String, String>,
    pub time: i64,
    pub presence: HashMap<String, Presence>,
    pub config: Config,
    pub contacts: Vec<Contact>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Drafts {
    pub chats: DraftSection,
    pub users: DraftSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftSection {
    pub saved: HashMap<String, String>,
    pub discarded: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatOptions {
    #[serde(rename = "SERVICE_CHAT", default)]
    pub service_chat: Option<bool>,
    #[serde(rename = "SIGN_ADMIN", default)]
    pub sign_admin: Option<bool>,
    #[serde(rename = "OFFICIAL", default)]
    pub official: Option<bool>,
    #[serde(rename = "MESSAGE_COPY_NOT_ALLOWED", default)]
    pub message_copy_not_allowed: Option<bool>,
    #[serde(rename = "ONLY_OWNER_CAN_CHANGE_ICON_TITLE", default)]
    pub only_owner_can_change_icon_title: Option<bool>,
    #[serde(rename = "ONLY_ADMIN_CAN_ADD_MEMBER", default)]
    pub only_admin_can_add_member: Option<bool>,
    #[serde(rename = "ONLY_ADMIN_CAN_CALL", default)]
    pub only_admin_can_call: Option<bool>,
    #[serde(rename = "SENT_BY_PHONE", default)]
    pub sent_by_phone: Option<bool>,
    #[serde(rename = "ALL_CAN_PIN_MESSAGE", default)]
    pub all_can_pin_message: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Presence {
    pub seen: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    #[serde(rename = "chatFolders")]
    pub chat_folders: ChatFolders,
    pub user: UserConfig,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(rename = "invite-short")]
    pub invite_short: String,
    #[serde(rename = "money-transfer-botid")]
    pub money_transfer_botid: i32,
    #[serde(rename = "set-unread-timeout")]
    pub set_unread_timeout: i32,
    #[serde(rename = "account-removal-enabled")]
    pub account_removal_enabled: bool,
    pub gce: bool,
    #[serde(rename = "image-size")]
    pub image_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatFolders {
    #[serde(rename = "FOLDERS")]
    pub folders: Vec<Folder>,
    #[serde(rename = "ALL_FILTER_EXCLUDE")]
    pub all_filter_exclude: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub favorites: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub emoji: Option<String>,
    pub id: String,
    pub filters: Vec<String>,
    #[serde(rename = "hideEmpty")]
    pub hide_empty: bool,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(rename = "SEARCH_BY_PHONE")]
    pub search_by_phone: String,
    #[serde(rename = "INCOMING_CALL")]
    pub incoming_call: String,
    #[serde(rename = "CHATS_PUSH_NOTIFICATION")]
    pub chats_push_notification: String,
}