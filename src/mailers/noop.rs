use async_trait::async_trait;

use crate::Mailer;
use crate::MailerError;
use crate::Message;

pub struct NoopMailer;

#[async_trait]
impl Mailer for NoopMailer {
    async fn send(&self, _: &Message) -> Result<Vec<String>, MailerError> {
        return Ok(Vec::new());
    }
}
