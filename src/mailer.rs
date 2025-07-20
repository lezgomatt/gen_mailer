use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::message::Message;

#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send(&self, message: &Message) -> Result<Vec<String>, MailerError>;
}

#[derive(Debug)]
pub enum MailerError {
    UnexpectedResponse(u16, String),
    UnexpectedError(Box<dyn Error>),
}

impl fmt::Display for MailerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            MailerError::UnexpectedResponse(status, body) => {
                write!(f, "Unexpected response: {} - {}", status, body)
            }
            MailerError::UnexpectedError(error) => write!(f, "Unexpected error: {}", error),
        };
    }
}

impl Error for MailerError {}

impl From<std::io::Error> for MailerError {
    fn from(err: std::io::Error) -> Self {
        return MailerError::UnexpectedError(Box::new(err));
    }
}

#[cfg(feature = "__reqwest")]
impl From<reqwest::Error> for MailerError {
    fn from(err: reqwest::Error) -> Self {
        return MailerError::UnexpectedError(Box::new(err));
    }
}

#[cfg(feature = "aws_ses")]
impl<E, R> From<aws_sdk_sesv2::error::SdkError<E, R>> for MailerError
where
    E: Error + 'static,
    R: fmt::Debug + 'static,
{
    fn from(err: aws_sdk_sesv2::error::SdkError<E, R>) -> Self {
        return MailerError::UnexpectedError(Box::new(err));
    }
}
