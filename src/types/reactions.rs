use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionRequest {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: ReactionRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionRequestPayload {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    #[serde(rename = "messageIds")]
    pub message_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: ReactionResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReactionResponsePayload {
    Success(ReactionResponseSuccess),
    Error(ErrorPayload),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionResponseSuccess {
    #[serde(rename = "messagesReactions")]
    pub messages_reactions: HashMap<String, MessageReactions>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MessageReactions {
    #[serde(default)]
    pub counters: Vec<ReactionCounter>,
    #[serde(rename = "yourReaction", default)]
    pub your_reaction: Option<String>,
    #[serde(rename = "totalCount", default)]
    pub total_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionCounter {
    pub count: i32,
    pub reaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetReactionRequest {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: SetReactionRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetReactionRequestPayload {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub reaction: Reaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "reactionType")]
    pub reaction_type: String,
    pub id: String,
}

// Response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct SetReactionResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: SetReactionResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetReactionResponsePayload {
    #[serde(rename = "reactionInfo")]
    pub reaction_info: Option<MessageReactions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveReactionRequest {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: RemoveReactionRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveReactionRequestPayload {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    #[serde(rename = "messageId")]
    pub message_id: String,
}

// Remove reaction response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveReactionResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: RemoveReactionResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveReactionResponsePayload {
    #[serde(rename = "reactionInfo")]
    pub reaction_info: Option<HashMap<String, Value>>, // Empty object in response
}