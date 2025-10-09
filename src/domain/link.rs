use chrono::{DateTime, Utc};
use rand::Rng;
use uuid::Uuid;

use crate::domain::errors::LinkError;
#[derive(Debug, Clone, PartialEq)]
pub struct LinkId(Uuid);

impl LinkId {
    pub fn value() -> Uuid {
        let raw_id = Uuid::new_v4();

        return raw_id;
    }

    pub fn from_string(raw: Uuid) -> Result<String, LinkError> {
        let raw_string = raw.to_string();

        let raw_string_trimmed = raw_string.trim();

        if raw_string_trimmed.is_empty() {
            return Err(LinkError::LinkIdNotFound);
        }

        let id_string = raw_string_trimmed.to_string();

        Ok(id_string)
    }
}

impl From<Uuid> for LinkId {
    fn from(value: Uuid) -> LinkId {
        LinkId(value)
    }
}

// OWASP A01
#[derive(Debug, Clone, PartialEq)]
pub struct LinkHashedCode {
    code: String,
}
impl LinkHashedCode {
    pub fn value(raw: String) -> Result<Self, LinkError> {
        let trimmed_raw: &str = raw.trim();

        if trimmed_raw.is_empty() {
            return Err(LinkError::EmptyHashedCode);
        }

        let hash_code = trimmed_raw.to_string();

        Ok(Self { code: hash_code })
    }
}

impl TryFrom<String> for LinkHashedCode {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Short url is empty.");
        }

        let item = value_trimmed.to_string();

        Ok(Self { code: item })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortUrl(String);

impl ShortUrl {
    pub fn value() -> Result<Self, LinkError> {
        let content_length: i32 = 7;

        let mut rng = rand::rng();

        let character_pick = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let char_pool: Vec<char> = character_pick.chars().collect();

        let char_merged: String = (0..content_length)
            .map(|_| {
                let idx = rng.random_range(0..char_pool.len());
                char_pool[idx]
            })
            .collect();

        let char_merged_trim = char_merged.trim();

        if char_merged_trim.is_empty() {
            return Err(LinkError::CodeGenerationFailure);
        }

        let short_url: String = char_merged_trim.to_string();

        Ok(Self(short_url))
    }
}

impl TryFrom<String> for ShortUrl {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Short url is empty.");
        }

        let item = value_trimmed.to_string();

        Ok(Self(item))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserUrl {
    raw: String,
}

impl UserUrl {
    pub fn value(&self) -> &String {
        &self.raw
    }
}

impl TryFrom<String> for UserUrl {
    type Error = &'static str;
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        let raw_trimmed = raw.trim();

        if raw_trimmed.is_empty() {
            return Err("Link is empty");
        }

        let raw = raw_trimmed.to_string();

        Ok(Self { raw })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    id: LinkId,
    code: LinkHashedCode,
    short_url: ShortUrl,
    user_url: UserUrl,
    created_at: DateTime<Utc>,
}

impl Link {
    pub fn new(
        id: Uuid,
        code: String,
        short_url: String,
        user_url: String,
        created_at: DateTime<Utc>,
    ) -> Result<Link, String> {
        let link_id = LinkId::from(id);
        let hash_code = LinkHashedCode::try_from(code)?;
        let generated_url = ShortUrl::try_from(short_url)?;
        let input_url = UserUrl::try_from(user_url)?;
        let creation_time = Utc::now();
        Ok(Self {
            id: link_id,
            code: hash_code,
            short_url: generated_url,
            user_url: input_url,
            created_at: creation_time,
        })
    }

    pub fn id(&self) -> &LinkId {
        &self.id
    }

    pub fn delete_hash_code(&self) -> &LinkHashedCode {
        &self.code
    }

    pub fn short_url(&self) -> &ShortUrl {
        &self.short_url
    }

    pub fn user_url(&self) -> &UserUrl {
        &self.user_url
    }

    pub fn created_at(self) -> DateTime<Utc> {
        self.created_at
    }
}
