pub mod console;
pub mod noop;

#[cfg(feature = "aws_ses")]
pub mod aws_ses;

#[cfg(feature = "sendgrid")]
pub mod sendgrid;
