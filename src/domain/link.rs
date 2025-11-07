use crate::domain::errors::LinkError;
use chrono::{DateTime, Utc};
use hex;
use rand::RngCore;
use rand::{rngs::OsRng, Rng};
use sha2::{Digest, Sha256};
use uuid::Uuid;

// OWASP A01 Broken Access Control
#[derive(Debug, Clone, PartialEq)]
pub struct LinkId(Uuid);

impl LinkId {
    pub fn generate() -> Uuid {
        Uuid::new_v4()
    }

    pub fn from_string(raw: String) -> Result<LinkId, LinkError> {
        let uuid = Uuid::parse_str(&raw).map_err(|_| LinkError::InvalidFormat)?;

        Ok(LinkId(uuid))
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for LinkId {
    fn from(value: Uuid) -> LinkId {
        LinkId(value)
    }
}

// OWASP A01
#[derive(Debug, Clone, PartialEq)]
pub struct LinkKey(String);

impl LinkKey {
    pub fn generate() -> Result<Self, LinkError> {
        let mut random_bytes = [0u8; 16];
        OsRng.fill_bytes(&mut random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(random_bytes);
        let hash_result = hasher.finalize();

        let full_hex = hex::encode(hash_result);

        let short_code = full_hex[0..8].to_string();

        if short_code.is_empty() {
            return Err(LinkError::EmptyHashedCode);
        }

        Ok(Self(short_code))
    }

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for LinkKey {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Hashed code is empty");
        }

        Ok(Self(value_trimmed.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortUrl(String);

impl ShortUrl {
    pub fn value() -> Result<Self, LinkError> {
        let content_length: i32 = 7;
        let mut rng = OsRng;

        let character_pick = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let char_pool: Vec<char> = character_pick.chars().collect();

        let char_merged: String = (0..content_length)
            .map(|_| {
                let idx = rng.gen_range(0..char_pool.len());
                char_pool[idx]
            })
            .collect();

        let char_merged_trim = char_merged.trim();

        if char_merged_trim.is_empty() {
            return Err(LinkError::CodeGenerationFailure);
        }

        Ok(Self(char_merged_trim.to_string()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for ShortUrl {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Short url is empty.");
        }

        Ok(Self(value_trimmed.to_string()))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserUrl {
    raw: String,
}

impl UserUrl {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }
    pub fn value(&self) -> &String {
        &self.raw
    }

    pub fn into_inner(self) -> String {
        self.raw
    }

    pub fn as_str(&self) -> &str {
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

        Ok(Self {
            raw: raw_trimmed.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatedAt(DateTime<Utc>);

impl CreatedAt {
    pub fn value() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        CreatedAt(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    id: LinkId,
    code: LinkKey,
    short_url: ShortUrl,
    user_url: UserUrl,
    created_at: CreatedAt,
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
        let hash_code = LinkKey::try_from(code)?;
        let generated_url = ShortUrl::try_from(short_url)?;
        let input_url = UserUrl::try_from(user_url)?;
        let creation_time = CreatedAt::from(created_at);

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

    pub fn delete_hash_code(&self) -> &LinkKey {
        &self.code
    }

    pub fn short_url(&self) -> &ShortUrl {
        &self.short_url
    }

    pub fn user_url(&self) -> &UserUrl {
        &self.user_url
    }

    pub fn created_at(self) -> CreatedAt {
        self.created_at
    }
}
