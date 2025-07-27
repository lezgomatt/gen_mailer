use async_trait::async_trait;

use crate::GenericMailer;
use crate::GenericMailerError;
use crate::Message;

pub struct NoOpMailer;

#[async_trait]
impl GenericMailer for NoOpMailer {
    async fn send(&self, _: &Message) -> Result<Vec<String>, GenericMailerError> {
        return Ok(Vec::new());
    }
}
