use chrono::{DateTime, Utc};
use rand::Rng;
use uuid::Uuid;

use super::errors::LinkError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Link {
    id: Uuid,
    long_url: String,
    short_code: String,
    created_at: DateTime<Utc>,
    // Cryptographic Failures (A02)
    delete_key_hash: String,
}

impl Link {
    pub fn new(
        long_url: String,
        short_code: String,
        delete_key_hash: String,
    ) -> Result<Self, LinkError> {
        // Input Validation (A08)
        if long_url.trim().is_empty() {
            return Err(LinkError::EmptyLongUrl);
        }

        if short_code.trim().is_empty() {
            return Err(LinkError::EmptyShortCode);
        }

        if delete_key_hash.trim().is_empty() {
            return Err(LinkError::EmptyDeleteKeyHash);
        }

        let now = Utc::now();

        Ok(Self {
            // Non-guessable ID (A01)
            id: Uuid::new_v4(),
            long_url,
            short_code,
            created_at: now,
            // (A02)
            delete_key_hash,
        })
    }

    pub fn generate_code() -> String {
        const LENGTH: usize = 7;
        let mut rng = rand::rng();

        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();

        (0..LENGTH)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
                chars[idx]
            })
            .collect()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn long_url(&self) -> &str {
        &self.long_url
    }

    pub fn short_code(&self) -> &str {
        &self.short_code
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn delete_key_hash(&self) -> &str {
        &self.delete_key_hash
    }
}
