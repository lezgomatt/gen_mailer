use crate::message::Message;

#[async_trait::async_trait]
pub trait Mailer: Send + Sync {
    async fn send(&self, message: &Message) -> Result<Vec<String>, MailerError>;
}

// TODO
pub struct MailerError {}
