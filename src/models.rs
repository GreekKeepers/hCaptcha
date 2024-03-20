use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HCaptchaResponse {
    pub success: bool,
    pub hostname: String,
}
