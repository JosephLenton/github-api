#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
}
