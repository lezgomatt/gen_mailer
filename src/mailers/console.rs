use std::io;

use async_trait::async_trait;

use crate::Address;
use crate::Mailer;
use crate::MailerError;
use crate::Message;

pub struct ConsoleMailer;

#[async_trait]
impl Mailer for ConsoleMailer {
    async fn send(&self, m: &Message) -> Result<Vec<String>, MailerError> {
        let _ = self.send(io::stdout(), m); // FIXME

        return Ok(Vec::new());
    }
}

impl ConsoleMailer {
    fn send(&self, mut w: impl io::Write, m: &Message) -> Result<(), std::io::Error> {
        writeln!(w, "============ [ BEGIN EMAIL ] ============")?;
        writeln!(w, "From: {}", m.from)?;

        if let Some(reply_to) = &m.reply_to {
            writeln!(w, "Reply-To: {}", reply_to)?;
        }

        if !m.to.is_empty() {
            writeln!(w, "To: {}", join_addresses(&m.to))?;
        }

        if !m.cc.is_empty() {
            writeln!(w, "Cc: {}", join_addresses(&m.cc))?;
        }

        if !m.bcc.is_empty() {
            writeln!(w, "Bcc: {}", join_addresses(&m.bcc))?;
        }

        writeln!(w, "Subject: {}", m.subject)?;
        writeln!(w)?;

        if let Some(body) = &m.text_body {
            writeln!(w, "{}", body)?;
        } else {
            writeln!(w, "No text body.")?;
        }

        writeln!(w, "============ [  END EMAIL  ] ============")?;

        return Ok(());
    }
}

fn join_addresses(addrs: &[Address]) -> String {
    return addrs
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join(", ");
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Address;
    use crate::MessageBuilder;

    #[test]
    fn test_console_mailer() {
        let mailer = ConsoleMailer;
        let message = MessageBuilder::new()
            .from(Address::with_name("Sender", "sender@example.com"))
            .to(Address::with_name("Recipient", "recipient@example.com"))
            .cc(Address::new("cc@example.com"))
            .subject("Test Email")
            .text_body("This is a test email.")
            .build()
            .unwrap();

        let expected = [
            "============ [ BEGIN EMAIL ] ============",
            "From: Sender <sender@example.com>",
            "To: Recipient <recipient@example.com>",
            "Cc: cc@example.com",
            "Subject: Test Email",
            "",
            "This is a test email.",
            "============ [  END EMAIL  ] ============",
            "",
        ]
        .join("\n");

        let mut actual = Vec::new();
        mailer.send(&mut actual, &message).unwrap();
        let actual = String::from_utf8(actual).unwrap();

        assert_eq!(actual, expected);
    }
}
