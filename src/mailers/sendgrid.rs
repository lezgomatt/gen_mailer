use async_trait::async_trait;
use serde_json::json;

use crate::Address;
use crate::GenericMailer;
use crate::GenericMailerError;
use crate::Message;

pub struct SendgridMailer {
    client: reqwest::Client,
    api_key: String,
}

impl SendgridMailer {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    pub fn with_client(client: reqwest::Client, api_key: impl Into<String>) -> Self {
        Self {
            client,
            api_key: api_key.into(),
        }
    }
}

#[async_trait]
impl GenericMailer for SendgridMailer {
    // See: https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send
    async fn send(&self, m: &Message) -> Result<Vec<String>, GenericMailerError> {
        let request = Self::build_request(m);

        let response = self
            .client
            .post("https://api.sendgrid.com/v3/mail/send")
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let body = response.text().await?;

            return Err(GenericMailerError::UnexpectedResponse(status_code, body));
        }

        // NOTE: x-message-id is not the same as the message ID,
        // but the message ID is prefixed with the x-message-id.
        // https://www.twilio.com/docs/sendgrid/glossary/what-is-x-message-id
        // https://www.twilio.com/docs/sendgrid/glossary/message-id
        let x_message_id = response
            .headers()
            .get("x-message-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        return Ok(x_message_id.into_iter().collect());
    }
}

impl SendgridMailer {
    fn build_request(m: &Message) -> serde_json::Value {
        let mut req = json!({
            "from": Self::build_address(&m.from),
            "personalizations": [Self::build_personalization(m)],
            "subject": m.subject,
            "content": Self::build_content(m),
        });

        if let Some(reply_to) = &m.reply_to {
            req["reply_to"] = Self::build_address(reply_to);
        }

        if !m.headers.is_empty() {
            let map = serde_json::Map::from_iter(
                m.headers.iter().map(|(k, v)| (k.to_string(), json!(v))),
            );
            req["headers"] = serde_json::Value::from(map);
        }

        if let Some(category) = &m.category {
            req["categories"] = json!([category]);
        }

        if !m.metadata.is_empty() {
            let map = serde_json::Map::from_iter(
                m.metadata.iter().map(|(k, v)| (k.to_string(), json!(v))),
            );
            req["custom_args"] = serde_json::Value::from(map);
        }

        return req;
    }

    fn build_personalization(m: &Message) -> serde_json::Value {
        let mut personalization = json!({
            "to": m.to.iter().map(Self::build_address).collect::<serde_json::Value>(),
        });

        if !m.cc.is_empty() {
            personalization["cc"] = m.cc.iter().map(Self::build_address).collect();
        }

        if !m.bcc.is_empty() {
            personalization["bcc"] = m.bcc.iter().map(Self::build_address).collect();
        }

        return personalization;
    }

    fn build_content(m: &Message) -> Vec<serde_json::Value> {
        let mut content = Vec::with_capacity(2);

        if let Some(body) = &m.text_body {
            content.push(json!({
                "type": "text/plain",
                "value": body,
            }));
        }

        if let Some(body) = &m.html_body {
            content.push(json!({
                "type": "text/html",
                "value": body,
            }));
        }

        return content;
    }

    fn build_address(addr: &Address) -> serde_json::Value {
        return if let Some(name) = &addr.name {
            json!({ "email": addr.email, "name": name })
        } else {
            json!({ "email": addr.email })
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendgrid_mailer() {
        let message = Message::builder()
            .from(Address::with_name("Sender", "sender@example.com"))
            .to(Address::with_name("Recipient", "recipient@example.com"))
            .cc(Address::new("cc@example.com"))
            .subject("Test Email")
            .text_body("This is a test email.")
            .build()
            .unwrap();

        let expected = json!({
            "from": { "name": "Sender", "email": "sender@example.com" },
            "personalizations": [
                {
                    "to": [{ "name": "Recipient", "email": "recipient@example.com" }],
                    "cc": [{ "email": "cc@example.com" }],
                },
            ],
            "subject": "Test Email",
            "content": [
                { "type": "text/plain", "value": "This is a test email." },
            ],
        });

        let actual = SendgridMailer::build_request(&message);

        assert_eq!(actual, expected);
    }
}
