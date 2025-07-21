use std::borrow::Cow;
use std::fmt;

use super::Address;

#[derive(Debug, Clone)]
pub struct Message<'a> {
    pub from: Address<'a>,
    pub reply_to: Option<Address<'a>>,
    pub to: Vec<Address<'a>>,
    pub cc: Vec<Address<'a>>,
    pub bcc: Vec<Address<'a>>,
    pub subject: Cow<'a, str>,
    pub text_body: Option<Cow<'a, str>>,
    pub html_body: Option<Cow<'a, str>>,
    // TODO:
    // pub attachments: Vec<MessageAttachment<'a>>,
    // pub inline_attachments: Vec<MessageAttachment<'a>>,
    // pub headers: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    // pub category: Cow<'a, str>,
    // pub metadata: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

// pub struct MessageAttachment<'a> {
//     pub name: Cow<'a, str>,
//     pub content_type: mime::Mime,
//     pub bytes: Cow<'a, [u8]>,
// }

impl Message<'_> {
    pub fn builder<'a>() -> MessageBuilder<'a> {
        return MessageBuilder::new();
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum MessageBuilderError {
    MissingFrom,
    MissingTo,
    MissingSubject,
    MissingBody,
}

impl fmt::Display for MessageBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            MessageBuilderError::MissingFrom => "Sender (`from`) is missing",
            MessageBuilderError::MissingTo => "Recipient (`to`) is missing",
            MessageBuilderError::MissingSubject => "Subject is missing",
            MessageBuilderError::MissingBody => {
                "Body is missing, provide at least one of `text_body` or `html_body`"
            }
        };

        return write!(f, "{message}");
    }
}

impl std::error::Error for MessageBuilderError {}

#[derive(Debug, Default)]
pub struct MessageBuilder<'a> {
    from: Option<Address<'a>>,
    reply_to: Option<Address<'a>>,
    to: Vec<Address<'a>>,
    cc: Vec<Address<'a>>,
    bcc: Vec<Address<'a>>,
    subject: Option<Cow<'a, str>>,
    text_body: Option<Cow<'a, str>>,
    html_body: Option<Cow<'a, str>>,
}

impl<'a> MessageBuilder<'a> {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn from(mut self, address: impl Into<Address<'a>>) -> Self {
        self.from = Some(address.into());
        return self;
    }

    pub fn reply_to(mut self, address: impl Into<Address<'a>>) -> Self {
        self.reply_to = Some(address.into());
        return self;
    }

    pub fn set_reply_to(mut self, address: Option<impl Into<Address<'a>>>) -> Self {
        self.reply_to = address.map(|addr| addr.into());
        return self;
    }

    pub fn to(mut self, address: impl Into<Address<'a>>) -> Self {
        self.to.push(address.into());
        return self;
    }

    pub fn set_to(mut self, addresses: Vec<impl Into<Address<'a>>>) -> Self {
        self.to = addresses.into_iter().map(|addr| addr.into()).collect();
        return self;
    }

    pub fn cc(mut self, address: impl Into<Address<'a>>) -> Self {
        self.cc.push(address.into());
        return self;
    }

    pub fn set_cc(mut self, addresses: Vec<impl Into<Address<'a>>>) -> Self {
        self.cc = addresses.into_iter().map(|addr| addr.into()).collect();
        return self;
    }

    pub fn bcc(mut self, address: impl Into<Address<'a>>) -> Self {
        self.bcc.push(address.into());
        return self;
    }

    pub fn set_bcc(mut self, addresses: Vec<impl Into<Address<'a>>>) -> Self {
        self.bcc = addresses.into_iter().map(|addr| addr.into()).collect();
        return self;
    }

    pub fn subject(mut self, subject: impl Into<Cow<'a, str>>) -> Self {
        self.subject = Some(subject.into());
        return self;
    }

    pub fn text_body(mut self, body: impl Into<Cow<'a, str>>) -> Self {
        self.text_body = Some(body.into());
        return self;
    }

    pub fn set_text_body(mut self, body: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.text_body = body.map(|b| b.into());
        return self;
    }

    pub fn html_body(mut self, body: impl Into<Cow<'a, str>>) -> Self {
        self.html_body = Some(body.into());
        return self;
    }

    pub fn set_html_body(mut self, body: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.html_body = body.map(|b| b.into());
        return self;
    }

    pub fn build(self) -> Result<Message<'a>, MessageBuilderError> {
        let from = self.from.ok_or(MessageBuilderError::MissingFrom)?;

        if self.to.is_empty() {
            return Err(MessageBuilderError::MissingTo);
        }

        let subject = self.subject.ok_or(MessageBuilderError::MissingSubject)?;

        if self.text_body.is_none() && self.html_body.is_none() {
            return Err(MessageBuilderError::MissingBody);
        }

        Ok(Message {
            from,
            reply_to: self.reply_to,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject,
            text_body: self.text_body,
            html_body: self.html_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_builder() {
        let message = Message::builder()
            .from("sender@example.com")
            .to("recipient@example.com")
            .subject("Test Email")
            .text_body("This is a test email.")
            .build()
            .unwrap();

        assert_eq!(message.from.email, "sender@example.com");
        assert_eq!(message.to.len(), 1);
        assert_eq!(message.to[0].email, "recipient@example.com");
        assert_eq!(message.subject, "Test Email");
        assert_eq!(message.text_body.as_deref(), Some("This is a test email."));

        // TODO: Test the optional fields
    }
}
