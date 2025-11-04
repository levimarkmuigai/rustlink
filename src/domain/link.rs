use crate::domain::errors::LinkError;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher};
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, Rng};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct LinkId(Uuid);

impl LinkId {
    pub fn value() -> Uuid {
        Uuid::new_v4()
    }

    pub fn from_string(raw: Uuid) -> Result<String, LinkError> {
        let raw_string = raw.to_string();
        let raw_string_trimmed = raw_string.trim();

        if raw_string_trimmed.is_empty() {
            return Err(LinkError::LinkIdNotFound);
        }

        Ok(raw_string_trimmed.to_string())
    }
}

impl From<Uuid> for LinkId {
    fn from(value: Uuid) -> LinkId {
        LinkId(value)
    }
}

// OWASP A01
#[derive(Debug, Clone, PartialEq)]
pub struct LinkHashedCode(String);

impl LinkHashedCode {
    pub fn value() -> Result<String, LinkError> {
        let mut rng = OsRng;
        let value: u64 = rng.gen();
        let mut sha256 = Sha256::new();

        sha256.update(value.to_string().as_bytes());
        let result = sha256.finalize();
        let hexed_result = hex::encode(result);

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let salted_hex_result = argon2
            .hash_password(hexed_result.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hashed_code_result = PasswordHash::new(&salted_hex_result).unwrap().to_string();

        if salted_hex_result.is_empty() {
            return Err(LinkError::EmptyHashedCode)?;
        }

        Ok(hashed_code_result)
    }
}

impl TryFrom<String> for LinkHashedCode {
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
}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        CreatedAt(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    id: LinkId,
    code: LinkHashedCode,
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
        let hash_code = LinkHashedCode::try_from(code)?;
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

    pub fn delete_hash_code(&self) -> &LinkHashedCode {
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
