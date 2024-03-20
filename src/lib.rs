mod models;
pub use models::*;

pub mod errors;

use reqwest::Client;
use std::sync::Arc;
use url;

pub struct HCaptcha {
    client: Arc<Client>,
    secret: String,
}

impl HCaptcha {
    pub fn new(secret: String) -> Self {
        Self {
            client: Arc::new(Client::new()),
            secret: url::form_urlencoded::byte_serialize(secret.as_bytes()).collect(),
        }
    }

    pub async fn verify(&self, token: String) -> Result<HCaptchaResponse, errors::Error> {
        let serialized_token: String =
            url::form_urlencoded::byte_serialize(token.as_bytes()).collect();
        let body = format!("response={}&secret={}", serialized_token, self.secret);
        let res = self
            .client
            .post("https://api.hcaptcha.com/siteverify")
            .body(body)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .send()
            .await
            .map_err(errors::Error::RequestError)?
            .text()
            .await
            .map_err(errors::Error::RequestError)?;

        let deserialized_res: HCaptchaResponse =
            serde_json::from_str(&res).map_err(|err| errors::Error::SerdeError(err, res))?;

        Ok(deserialized_res)
    }
}
