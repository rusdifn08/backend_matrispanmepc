use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageChannel {
    WhatsApp,
    WebInquiry,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageStatus {
    Unread,
    Read,
    Replied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMessage {
    pub id: String,
    pub sender_name: String,
    pub sender_company: Option<String>,
    pub sender_phone: String,
    pub sender_email: Option<String>,
    pub channel: MessageChannel,
    pub subject: String,
    pub content: String,
    pub ref_code: Option<String>,
    pub status: MessageStatus,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMessagePayload {
    pub sender_name: String,
    pub sender_company: Option<String>,
    pub sender_phone: String,
    pub sender_email: Option<String>,
    pub channel: MessageChannel,
    pub subject: String,
    pub content: String,
    pub ref_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReplyMessagePayload {
    pub reply_content: String,
    pub admin_user: String,
}
