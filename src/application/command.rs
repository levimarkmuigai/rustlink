#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    raw: String,
}

impl Url {

    pub fn new(input: String) -> Result<Url, String> {

        Ok(Self{ raw: input})
    }

    pub fn url(&self) -> &Url {
        &self
    }

}
