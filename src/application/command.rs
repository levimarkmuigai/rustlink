#[derive(Debug, Clone)]
pub struct CreateLinkCommand {
    pub long_url: String,

    // User provided secret used to manage link (A02)
    pub delete_key: String,
}

#[derive(Debug, Clone)]
pub struct LinkCreationResult {
    pub short_code: String,

    // User provided secret used to manage short code (A02)
    pub delete_key: String,
}
