mod console;
pub use console::ConsoleMailer;

mod noop;
pub use noop::NoopMailer;

#[cfg(feature = "aws_ses")]
pub mod aws_ses;
#[cfg(feature = "aws_ses")]
pub use aws_ses::AwsSesMailer;

#[cfg(feature = "mailtrap")]
pub mod mailtrap;
#[cfg(feature = "mailtrap")]
pub use mailtrap::MailtrapMailer;

#[cfg(feature = "sendgrid")]
pub mod sendgrid;
#[cfg(feature = "sendgrid")]
pub use sendgrid::SendgridMailer;
