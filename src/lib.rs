use serde_json::{json, Value};
use tungstenite::{connect, Message};
use url::Url;
use uuid::Uuid;
use std::collections::HashMap;
use std::io::{self, Write};
use std::{fs, process};

mod types;

struct MaxClient {
    id: Option<i64>,
    phone_number: String,
    auth_token: Option<String>,
    websocket: Option<tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>>,
}

impl MaxClient {
    pub fn new() -> Self {
        // EVERYTHING IS INTERCONNECTED
        println!("Welcome to MadLib 0.1!\nRemember: everything interconnected\n");
        MaxClient {
            id: None,
            phone_number: String::new(),
            auth_token: None,
            websocket: None,
        }
    }

    // Generates a user agent string for the WebSocket connection.
    // Note: You can provide any device type like Android or IOS and get device based code with <#> prefix
    fn generate_user_agent(&self) -> String {
        json!({
            "ver": 11,
            "cmd": 0,
            "seq": 0,
            "opcode": 6,
            "payload": {
                "userAgent": {
                    "deviceType": "WEB",
                    "locale": "ru_RU",
                    "osVersion": "Linux",
                    "deviceName": "Firefox",
                    "headerUserAgent": "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:135.0) Gecko/20100101 Firefox/135.0",
                    "deviceLocale": "ru-RU",
                    "appVersion": "4.8.42",
                    "screen": "1080x1920 1.0x",
                    "timezone": "Europe/Moscow"
                },
                "deviceId": Uuid::new_v4().to_string()
            }
        }).to_string()
    }

    // Connects to the WebSocket server.
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (socket, _) = connect(
            Url::parse("wss://ws-api.oneme.ru/websocket")?
        )?;
        
        self.websocket = Some(socket);
        
        let user_agent = self.generate_user_agent();
        
        if let Some(ws) = &mut self.websocket {
            ws.send(Message::Text(user_agent))?;
            ws.read()?;
        }

        Ok(())
    }

    pub fn disconnect(&mut self) {
        if let Some(ws) = &mut self.websocket {
            let _ = ws.close(None);
        }
        self.websocket = None;
    }

    // Terminates the client and disconnects from the server.
    // Unused, but can be useful for debugging.
    #[allow(dead_code)]
    fn die(&mut self) {
        self.disconnect();
        process::exit(1);
    }

    // Authenticates the user by phone number.
    pub fn authenticate(&mut self, phone_number: String) -> Result<String, Box<dyn std::error::Error>> {
        self.phone_number = phone_number;
        if self.websocket.is_none() {
            self.connect()?;
        }

        // Request auth code
        if let Some(ws) = &mut self.websocket {
            let auth_request = json!({
                "ver": 11,
                "cmd": 0,
                "seq": 3,
                "opcode": 17,
                "payload": {
                    "phone": &self.phone_number,
                    "type": "START_AUTH",
                    "language": "ru"
                }
            }).to_string();

            ws.send(Message::Text(auth_request))?;
            
            let code_resp = ws.read()?;
            let code_resp: Value = serde_json::from_str(code_resp.to_text()?)?;

            if let Some(error) = code_resp["payload"]["error"].as_str() {
                let message = code_resp["payload"]["localizedMessage"].as_str().unwrap_or("");
                return Err(format!("{}: {}", error, message).into());
            }

            let token = code_resp["payload"]["token"].as_str()
                .ok_or("Failed to get token")?;

            println!("Auth token received. Please enter the code sent to your phone.\n");
            
            print!("Auth code: ");
            io::stdout().flush()?;
            let mut code = String::new();
            io::stdin().read_line(&mut code)?;
            let code = code.trim();

            let verify_request = json!({
                "ver": 11,
                "cmd": 0,
                "seq": 1,
                "opcode": 18,
                "payload": {
                    "token": token,
                    "verifyCode": code,
                    "authTokenType": "CHECK_CODE"
                }
            }).to_string();

            ws.send(Message::Text(verify_request))?;
            
            let token_resp = ws.read()?;
            let token_resp: Value = serde_json::from_str(token_resp.to_text()?)?;
            
            self.auth_token = Some(
                token_resp["payload"]["tokenAttrs"]["LOGIN"]["token"]
                    .as_str()
                    .ok_or("Failed to get auth token")?
                    .to_string()
            );
            self.id = Some(
                token_resp["payload"]["profile"]["contact"]["id"]
                .as_i64()
                .ok_or("Failed to get user id")
                .unwrap()
            );
        }
        Ok(self.auth_token.clone().unwrap())
    }

    // Saves the auth token to a session file.
    pub fn save_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(token) = &self.auth_token {
            fs::write("mad.session", token)?;
        }
        Ok(())
    }

    // Loads the auth token from the session file.
    pub fn load_token(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match fs::read_to_string("mad.session") {
            Ok(token) => {
                let token = token.trim().to_string();
                if !token.is_empty() {
                    self.auth_token = Some(token.clone());
                    Ok(Some(token))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }

    // Authenticates the user by loading the token String.
    // MaxClient.get_chats() alias
    pub fn auth_by_token(&mut self, auth_token: String) -> Result<bool, Box<dyn std::error::Error>> {
        match self.get_chats(Some(auth_token)) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    // Retrieves messages from a chat.
    // If `from_time` is None, it defaults to the current time minus 9 hours.
    pub fn get_messages(&mut self, chat_id: i64, from_time: Option<i64>, backward: i32) -> Result<Vec<types::messages::Message>, Box<dyn std::error::Error>> {
        if self.websocket.is_none() {
            self.connect()?;
        }

        let default_from_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64 - (9 * 60 * 60 * 1000);

        let from_time = from_time.unwrap_or(default_from_time);

        if let Some(ws) = &mut self.websocket {
            let request = types::messages::MessageRequest {
                ver: 11,
                cmd: 0,
                seq: 1,
                opcode: 49,
                payload: types::messages::MessageRequestPayload {
                    chat_id,
                    from: from_time,
                    forward: 0,
                    backward,
                    get_messages: true,
                },
            };

            let request_json = serde_json::to_string(&request)?;
            ws.send(Message::Text(request_json))?;
            
            let response = ws.read()?;
            let response: types::messages::MessageResponse = serde_json::from_str(response.to_text()?)?;
            
            match response.payload {
                types::messages::MessageResponsePayload::Success(success) => Ok(success.messages),
                types::messages::MessageResponsePayload::Error(error) => {
                    Err(format!("Server error: {} - {}", error.error, error.message).into())
                }
            }
        } else {
            Err("WebSocket not connected".into())
        }
    }

    // Retrieves reactions for a list of messages in a chat.
    pub fn get_message_reactions(&mut self, chat_id: i64, message_ids: Vec<String>) -> Result<HashMap<String, types::reactions::MessageReactions>, Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.websocket {
            let request = types::reactions::ReactionRequest {
                ver: 11,
                cmd: 0,
                seq: 1,
                opcode: 180,
                payload: types::reactions::ReactionRequestPayload {
                    chat_id,
                    message_ids,
                },
            };

            let request_json = serde_json::to_string(&request)?;
            ws.send(Message::Text(request_json))?;
            
            let response = ws.read()?;
            let response: types::reactions::ReactionResponse = serde_json::from_str(response.to_text()?)?;
            
            match response.payload {
                types::reactions::ReactionResponsePayload::Success(success) => Ok(success.messages_reactions),
                types::reactions::ReactionResponsePayload::Error(error) => {
                    Err(format!("Server error: {} - {}", error.error, error.message).into())
                }
            }
        } else {
            Err("WebSocket not connected".into())
        }
    }

    // Uploads a photo to the server and returns the photo token.
    pub async fn upload_photo(&mut self, image_buffer: Vec<u8>, file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        if self.websocket.is_none() {
            self.connect()?;
        }

        if let Some(ws) = &mut self.websocket {
            let request = types::upload::UploadRequest {
                ver: 11,
                cmd: 0,
                seq: 1,
                opcode: 80,
                payload: types::upload::UploadRequestPayload {
                    count: 1,
                },
            };

            let request_json = serde_json::to_string(&request)?;
            ws.send(Message::Text(request_json))?;
            
            let response = ws.read()?;
            let upload_response: types::upload::UploadResponse = serde_json::from_str(response.to_text()?)?;
            
            match upload_response.payload {
                types::upload::UploadResponsePayload::Success(success) => {
                    let form = reqwest::multipart::Form::new()
                        .part("file", 
                            reqwest::multipart::Part::bytes(image_buffer)
                                .file_name(file_name.to_string())
                                .mime_str("image/png")?
                        );

                    let client = reqwest::Client::new();
                    let response: reqwest::Response = client
                        .post(&success.url)
                        .multipart(form)
                        .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:135.0) Gecko/20100101 Firefox/135.0")
                        .header("Origin", "https://web.max.ru")
                        .header("Referer", "https://web.max.ru/")
                        .send()
                        .await?;

                    let photo_response_dirty = response.text().await?;
                    let photo_response: types::upload::PhotoUploadResponse = serde_json::from_str(&photo_response_dirty)?;
                    
                    match photo_response {
                        types::upload::PhotoUploadResponse::Success(success) => {
                            let token = success.photos
                                .values()
                                .next()
                                .ok_or("No photo info in response")?
                                .token
                                .clone();
                            Ok(token)
                        },
                        types::upload::PhotoUploadResponse::Error(error) => {
                            Err(format!("Photo upload error: {} - {}", error.error, error.message).into())
                        }
                    }
                },
                types::upload::UploadResponsePayload::Error(error) => {
                    Err(format!("Upload URL error: {} - {}", error.error, error.message).into())
                }
            }
        } else {
            Err("WebSocket not connected".into())
        }
    }

    // Retrieves the list of chats.
    // Also used for authentication.
    pub fn get_chats(&mut self, auth_token: Option<String>) -> Result<types::chats::ChatsPayloadSuccess, Box<dyn std::error::Error>> {
        if let Some(token) = auth_token {
            self.auth_token = Some(token);
        }

        if self.auth_token.is_none() {
            return Err("No auth token provided. Please authenticate first.".into());
        }

        if self.websocket.is_none() {
            self.connect()?;
        }

        if let Some(ws) = &mut self.websocket {
            let request = json!({
                "ver": 11,
                "cmd": 0,
                "seq": 1,
                "opcode": 19,
                "payload": {
                    "interactive": true,
                    "token": &self.auth_token,
                    "chatsSync": 0,
                    "contactsSync": 0,
                    "presenceSync": 0,
                    "draftsSync": 0,
                    "chatsCount": 40
                }
            }).to_string();

            ws.send(Message::Text(request))?;
            let response = ws.read()?;
            let response: types::chats::ChatsResponse = serde_json::from_str(&response.to_string())?;
            
            match response.payload {
                types::chats::ChatsResponsePayload::Success(success) => Ok(success),
                types::chats::ChatsResponsePayload::Error(error) => {
                    Err(format!("Server error: {} - {}", error.error, error.message).into())
                }
            }
        } else {
            Err("WebSocket not connected".into())
        }
    }

    /// Sends a message to a chat.
    pub fn send_message(&mut self, chat_id: i64, message: types::messages::MessageBuilder) -> Result<types::messages::SendMessageResponse, Box<dyn std::error::Error>> {
        if self.websocket.is_none() {
            self.connect()?;
        }

        if let Some(ws) = &mut self.websocket {
            let request = message.build_request(chat_id);
            
            let request_json = serde_json::to_string(&request)?;
            ws.send(Message::Text(request_json))?;
            
            let response = ws.read()?;
            let response_text = response.to_text()?;
            
            let response: types::messages::SendMessageResponse = serde_json::from_str(&response_text)?;
            
            Ok(response)
        } else {
            Err("WebSocket not connected".into())
        }
    }
    
    // Sets a reaction to a message in a chat.
    pub fn set_reaction(
        &mut self,
        chat_id: i64,
        message_id: impl Into<String>,
        emoji: impl Into<String>
    ) -> Result<types::reactions::SetReactionResponse, Box<dyn std::error::Error>> {
        if self.websocket.is_none() {
            self.connect()?;
        }

        let message_id_str = message_id.into();

        if let Some(ws) = &mut self.websocket {

            let request = types::reactions::SetReactionRequest {
                ver: 11,
                cmd: 0,
                seq: 1,
                opcode: 178,
                payload: types::reactions::SetReactionRequestPayload {
                    chat_id,
                    message_id: message_id_str,
                    reaction: types::reactions::Reaction {
                        reaction_type: "EMOJI".to_string(),
                        id: emoji.into(),
                    },
                },
            };

            let request_json = serde_json::to_string(&request)?;
            ws.send(Message::Text(request_json))?;
            
            let response = ws.read()?;
            let response: types::reactions::SetReactionResponse = serde_json::from_str(response.to_text()?)?;
            
            Ok(response)
        } else {
            Err("WebSocket not connected".into())
        }
    }

    // Removes a reaction from a message in a chat.
    pub fn remove_reaction(
        &mut self,
        chat_id: i64,
        message_id: impl Into<String>
    ) -> Result<types::reactions::RemoveReactionResponse, Box<dyn std::error::Error>> {
        if self.websocket.is_none() {
            self.connect()?;
        }

        if let Some(ws) = &mut self.websocket {
            let request = types::reactions::RemoveReactionRequest {
                ver: 11,
                cmd: 0,
                seq: 1,
                opcode: 179,
                payload: types::reactions::RemoveReactionRequestPayload {
                    chat_id,
                    message_id: message_id.into(),
                },
            };

            let request_json = serde_json::to_string(&request)?;
            ws.send(Message::Text(request_json))?;
            
            let response = ws.read()?;
            let response: types::reactions::RemoveReactionResponse = serde_json::from_str(response.to_text()?)?;
            
            Ok(response)
        } else {
            Err("WebSocket not connected".into())
        }
    }
   
}