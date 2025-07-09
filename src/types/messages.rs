use serde::{Deserialize, Serialize};
//use std::collections::HashMap;

use crate::types::chats;
use crate::types::reactions;



#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRequest {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: MessageRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRequestPayload {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    pub from: i64,
    pub forward: i32,
    pub backward: i32,
    #[serde(rename = "getMessages")]
    pub get_messages: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: MessageResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageResponsePayload {
    Success(MessageResponseSuccess),
    Error(ErrorPayload),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponseSuccess {
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub sender: i64,
    #[serde(rename = "reactionInfo")]
    pub reaction_info: Option<reactions::MessageReactions>,
    #[serde(default)]
    pub link: Option<GetMessageLink>,
    pub id: String,
    pub time: i64,
    pub text: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub cid: Option<i64>,
    pub attaches: Option<Vec<chats::Attachment>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMessageLink {
    #[serde(rename = "type")]
    pub link_type: String,
    pub message: Box<Message>,
    #[serde(rename = "chatId")]
    pub chat_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preview {
    pub duration: Option<i32>,
    #[serde(rename = "previewData")]
    pub preview_data: String,
    pub thumbnail: Option<String>,
    #[serde(rename = "_type")]
    pub preview_type: String,
    pub width: i32,
    #[serde(rename = "videoId", default)]
    pub video_id: i64,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: SendMessagePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessagePayload {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    pub message: SendMessageContent,
    pub notify: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageContent {
    pub text: String,
    pub cid: i64,
    pub elements: Vec<String>,
    pub link: Option<MessageLink>,
    pub attaches: Vec<SendMessageAttachment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageAttachment {
    #[serde(rename = "_type")]
    pub attachment_type: String,
    #[serde(rename = "photoToken")]
    pub photo_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageLink {
    #[serde(rename = "type")]
    pub link_type: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
}
pub struct MessageBuilder {
    text: String,
    cid: i64,
    elements: Vec<String>,
    attaches: Vec<SendMessageAttachment>,
    link: Option<MessageLink>,
    notify: bool,
}

impl MessageBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            cid: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            elements: Vec::new(),
            attaches: Vec::new(),
            notify: true,
            link: None,
        }
    }

    pub fn reply_to(mut self, message_id: impl Into<String>) -> Self {
        self.link = Some(MessageLink {
            link_type: "REPLY".to_string(),
            message_id: message_id.into(),
        });
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn image(mut self, photo_token: impl Into<String>) -> Self {
        self.attaches.push(SendMessageAttachment {
            attachment_type: "PHOTO".to_string(),
            photo_token: photo_token.into(),
        });
        self
    }

    pub fn silent(mut self) -> Self {
        self.notify = false;
        self
    }

    pub fn build_request(&self, chat_id: i64) -> SendMessageRequest {
        SendMessageRequest {
            ver: 11,
            cmd: 0,
            seq: 1,
            opcode: 64,
            payload: SendMessagePayload {
                chat_id,
                message: SendMessageContent {
                    text: self.text.clone(),
                    cid: self.cid,
                    elements: self.elements.clone(),
                    attaches: self.attaches.clone(),
                    link: self.link.clone(),
                },
                notify: self.notify,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: SendMessageResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponsePayload {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    pub message: SendMessageResponseMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponseMessage {
    pub sender: i64,
    pub id: String,
    pub time: i64,
    pub text: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub cid: i64,
    pub attaches: Vec<SendMessageResponseAttachment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponseAttachment {
    #[serde(rename = "previewData")]
    pub preview_data: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "photoToken")]
    pub photo_token: String,
    #[serde(rename = "_type")]
    pub attachment_type: String,
    pub width: i32,
    #[serde(rename = "photoId")]
    pub photo_id: i64,
    pub height: i32,
}