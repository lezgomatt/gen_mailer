use async_trait::async_trait;
use serde_json::json;

use crate::Address;
use crate::GenericMailer;
use crate::GenericMailerError;
use crate::Message;

pub struct MailtrapMailer {
    client: reqwest::Client,
    api_token: String,
}

impl MailtrapMailer {
    pub fn new(api_token: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_token: api_token.into(),
        }
    }

    pub fn with_client(client: reqwest::Client, api_token: impl Into<String>) -> Self {
        Self {
            client,
            api_token: api_token.into(),
        }
    }
}

#[async_trait]
impl GenericMailer for MailtrapMailer {
    // See: https://api-docs.mailtrap.io/docs/mailtrap-api-docs/67f1d70aeb62c-send-email-including-templates
    async fn send(&self, m: &Message) -> Result<Vec<String>, GenericMailerError> {
        let request = Self::build_request(m);

        let response = self
            .client
            .post("https://send.api.mailtrap.io/api/send")
            // Both `Api-Token: {api_token}` and `Authorization: Bearer {api_token}` are allowed
            .bearer_auth(&self.api_token)
            .json(&request)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let body = response.text().await?;

            return Err(GenericMailerError::UnexpectedResponse(status_code, body));
        }

        let status_code = response.status().as_u16();
        let text = response.text().await?;
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
            return Err(GenericMailerError::UnexpectedResponse(status_code, text));
        };

        let Some(true) = json.as_object().and_then(|o| o.get("success")?.as_bool()) else {
            return Err(GenericMailerError::UnexpectedResponse(status_code, text));
        };

        let Some(json_message_ids) = json
            .as_object()
            .and_then(|o| o.get("message_ids")?.as_array())
        else {
            return Err(GenericMailerError::UnexpectedResponse(status_code, text));
        };

        let ids = json_message_ids
            .iter()
            .flat_map(|id| id.as_str())
            .map(|s| s.to_string())
            .collect();

        return Ok(ids);
    }
}

impl MailtrapMailer {
    fn build_request(m: &Message) -> serde_json::Value {
        let mut req = json!({
            "from": Self::build_address(&m.from),
            "to": m.to.iter().map(Self::build_address).collect::<serde_json::Value>(),
            "subject": &m.subject,
        });

        if !m.cc.is_empty() {
            req["cc"] = m.cc.iter().map(Self::build_address).collect();
        }

        if !m.bcc.is_empty() {
            req["bcc"] = m.bcc.iter().map(Self::build_address).collect();
        }

        if let Some(reply_to) = &m.reply_to {
            req["reply_to"] = Self::build_address(reply_to);
        }

        if let Some(body) = &m.text_body {
            req["text"] = json!(body);
        }

        if let Some(body) = &m.html_body {
            req["html"] = json!(body);
        }

        return req;
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

    use crate::MessageBuilder;

    #[test]
    fn test_mailtrap_mailer() {
        let message = MessageBuilder::new()
            .from(Address::with_name("Sender", "sender@example.com"))
            .to(Address::with_name("Recipient", "recipient@example.com"))
            .cc(Address::new("cc@example.com"))
            .subject("Test Email")
            .text_body("This is a test email.")
            .build()
            .unwrap();

        let expected = json!({
            "from": { "name": "Sender", "email": "sender@example.com" },
            "to": [{ "name": "Recipient", "email": "recipient@example.com" }],
            "cc": [{ "email": "cc@example.com" }],
            "subject": "Test Email",
            "text": "This is a test email.",
        });

        let actual = MailtrapMailer::build_request(&message);

        assert_eq!(actual, expected);
    }
}
