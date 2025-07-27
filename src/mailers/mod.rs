mod console;
pub use console::ConsoleMailer;

mod no_op;
pub use no_op::NoOpMailer;

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
