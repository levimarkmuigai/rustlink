#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    raw: String,
}

impl Url {
    pub fn new(value: String) -> Result<Url, String> {
        let value_trim = value.trim();

        if value_trim.is_empty() {
            return Err("Empty input".to_string());
        }

        let raw = value_trim.to_string();

        Ok(Self { raw: raw })
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}
