use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadRequest {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: UploadRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadRequestPayload {
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub ver: i32,
    pub cmd: i32,
    pub seq: i32,
    pub opcode: i32,
    pub payload: UploadResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResponsePayload {
    Success(UploadResponseSuccess),
    Error(ErrorPayload),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponseSuccess {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhotoUploadResponse {
    Success(PhotoUploadSuccess),
    Error(ErrorPayload),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoUploadSuccess {
    pub photos: HashMap<String, PhotoInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoInfo {
    pub token: String,
}