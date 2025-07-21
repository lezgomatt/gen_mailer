use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::message::Message;

#[async_trait]
pub trait GenericMailer: Send + Sync {
    async fn send(&self, message: &Message) -> Result<Vec<String>, GenericMailerError>;
}

#[derive(Debug)]
pub enum GenericMailerError {
    UnexpectedResponse(u16, String),
    UnexpectedError(Box<dyn Error>),
}

impl fmt::Display for GenericMailerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            GenericMailerError::UnexpectedResponse(status, body) => {
                write!(f, "Unexpected response: {} - {}", status, body)
            }
            GenericMailerError::UnexpectedError(error) => write!(f, "Unexpected error: {}", error),
        };
    }
}

impl Error for GenericMailerError {}

impl From<std::io::Error> for GenericMailerError {
    fn from(err: std::io::Error) -> Self {
        return GenericMailerError::UnexpectedError(Box::new(err));
    }
}

#[cfg(feature = "__reqwest")]
impl From<reqwest::Error> for GenericMailerError {
    fn from(err: reqwest::Error) -> Self {
        return GenericMailerError::UnexpectedError(Box::new(err));
    }
}

#[cfg(feature = "aws_ses")]
impl<E, R> From<aws_sdk_sesv2::error::SdkError<E, R>> for GenericMailerError
where
    E: Error + 'static,
    R: fmt::Debug + 'static,
{
    fn from(err: aws_sdk_sesv2::error::SdkError<E, R>) -> Self {
        return GenericMailerError::UnexpectedError(Box::new(err));
    }
}
