use rocket::futures::executor::block_on;
use serde::{Serialize, Deserialize};
use serde_json::{Error, Value, json};
use std::{collections::HashMap, time::Duration};
use reqwest::*;
use reqwest::header;
use serde_json::Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stripe {
    pub key: String,
    pub secret: String,
}
impl  Stripe  {
    pub fn new(key: String, secret: String) -> Stripe {
        Stripe { key, secret }
    }
    pub async fn get_customer(&self, id: String) -> Result<Value> {
        // getCustomer request using stripe api directly
            // .header("Authorization", self.secret.clone().as_str())
            let mut headers = header::HeaderMap::new();
            // Consider marking security-sensitive headers with `set_sensitive`.
            let mut auth_value = header::HeaderValue::from_str(format!("Bearer {}", &self.secret.clone()).as_str()).unwrap();
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);
            let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(900))
            .default_headers(headers).build().expect("client faild to build!");
            client.get(format!("https://api.stripe.com/v1/customers/{}", id.as_str())).send().await.expect("faild get_customer")
            .json::<Value>().await
    }
}
