mod address;
mod generic_mailer;
mod message;

pub mod mailers;

pub use address::Address;
pub use generic_mailer::GenericMailer;
pub use generic_mailer::GenericMailerError;
pub use message::Message;
pub use message::MessageBuilder;
