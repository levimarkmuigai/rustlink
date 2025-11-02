#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    raw: String,
}

impl Url {
    pub fn new() -> Result<Url, String> {
        let input: String = Default::default();

        let input_trim = input.trim();

        if input_trim.is_empty() {
            return Err("Empty input".to_string());
        }

        let raw = input_trim.to_string();

        Ok(Self { raw: raw })
    }

    pub fn as_str(self) -> String {
        self.raw
    }
}
