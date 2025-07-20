mod address;
mod mailer;
mod message;

pub mod mailers;

pub use address::Address;
pub use mailer::Mailer;
pub use mailer::MailerError;
pub use message::Message;
pub use message::MessageBuilder;
