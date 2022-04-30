use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use thiserror::Error;

pub type OsmError = minreq::Error;

fn create_req(action: OsmAction) -> minreq::Request {
    minreq::get("https://www.1secmail.com/api/v1/").with_param("action", action.to_string())
}

pub fn get_domain_list() -> Result<Vec<String>, OsmError> {
    create_req(OsmAction::GetDomainList).send()?.json()
}

pub fn get_messages(mail_addr: &MailAddr) -> Result<Vec<MessageInfo>, OsmError> {
    create_req(OsmAction::GetMessages)
        .with_param("login", &mail_addr.login)
        .with_param("domain", &mail_addr.domain)
        .send()?
        .json()
}

pub fn read_message(mail_addr: &MailAddr, message_id: MessageId) -> Result<Message, OsmError> {
    create_req(OsmAction::ReadMessage)
        .with_param("login", &mail_addr.login)
        .with_param("domain", &mail_addr.domain)
        .with_param("id", &message_id.to_string())
        .send()?
        .json()
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct MessageId(u64);
impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct MessageInfo {
    pub id: MessageId,

    #[serde(with = "serde_with::rust::display_fromstr")]
    pub from: MailAddr,

    pub subject: String,

    #[serde(with = "osm_date_format")]
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Message {
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub from: MailAddr,

    pub subject: String,

    #[serde(with = "osm_date_format")]
    pub date: DateTime<Utc>,

    pub attachments: Vec<AttachmentInfo>,

    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentInfo {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MailAddr {
    pub login: String,
    pub domain: String,
}

impl FromStr for MailAddr {
    type Err = ParseMailAddrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let at_sign_index = s.find('@').ok_or(ParseMailAddrError::MissingAtSign)?;
        let login = &s[..at_sign_index];
        let domain = &s[at_sign_index + 1..];
        if login.is_empty() {
            return Err(ParseMailAddrError::EmptyLogin);
        }
        if domain.is_empty() {
            return Err(ParseMailAddrError::EmptyDomain);
        }
        return Ok(Self {
            login: login.to_string(),
            domain: domain.to_string(),
        });
    }
}

impl std::fmt::Display for MailAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.login, self.domain)
    }
}

#[derive(Debug, Error)]
pub enum ParseMailAddrError {
    #[error("missing '@' sign")]
    MissingAtSign,

    #[error("the login part of the mail address is empty")]
    EmptyLogin,

    #[error("the domain part of the mail address is empty")]
    EmptyDomain,
}

#[derive(Debug, Display)]
#[strum(serialize_all = "camelCase")]
enum OsmAction {
    GetDomainList,
    GetMessages,
    ReadMessage,
}

mod osm_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
