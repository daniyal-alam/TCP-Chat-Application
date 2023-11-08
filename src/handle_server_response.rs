#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub code: u32,
    pub message: String,
}