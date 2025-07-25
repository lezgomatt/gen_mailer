use async_trait::async_trait;
use aws_sdk_sesv2::types::Body;
use aws_sdk_sesv2::types::Content;
use aws_sdk_sesv2::types::Destination;
use aws_sdk_sesv2::types::EmailContent;
use aws_sdk_sesv2::types::Message as SesMessage;
use aws_sdk_sesv2::types::MessageHeader;
use aws_sdk_sesv2::types::MessageTag;

use crate::Address;
use crate::GenericMailer;
use crate::GenericMailerError;
use crate::Message;

pub struct AwsSesMailer {
    pub client: aws_sdk_sesv2::Client,
    pub configuration_set: Option<String>,
    pub sender_identity_arn: String,
    pub feedback_forwarding: Option<AwsSesFeedbackForwarding>,
}

pub struct AwsSesFeedbackForwarding {
    pub identity_arn: String,
    pub email: Address<'static>,
}

#[async_trait]
impl GenericMailer for AwsSesMailer {
    async fn send(&self, m: &Message) -> Result<Vec<String>, GenericMailerError> {
        let mut builder = self.client.send_email();

        if let Some(config) = &self.configuration_set {
            builder = builder.configuration_set_name(config);
        }

        if let Some(ff) = &self.feedback_forwarding {
            builder = builder
                .feedback_forwarding_email_address_identity_arn(&ff.identity_arn)
                .feedback_forwarding_email_address(ff.email.to_string())
        }

        builder = builder
            .from_email_address_identity_arn(&self.sender_identity_arn)
            .from_email_address(m.from.to_string());

        if let Some(reply_to) = &m.reply_to {
            builder = builder.reply_to_addresses(reply_to.to_string());
        }

        let destination = Self::build_destination(m);
        builder = builder.destination(destination);

        let content = Self::build_content(m);
        builder = builder.content(content);

        if m.category.is_some() || !m.headers.is_empty() {
            let mut tags = vec![];

            if let Some(category) = &m.category {
                tags.push(build_tag("category", category));
            }

            tags.reserve(m.headers.len());
            for (k, v) in &m.headers {
                tags.push(build_tag(k, v));
            }

            builder = builder.set_email_tags(Some(tags));
        }

        let response = builder.send().await?;

        return Ok(response.message_id.into_iter().collect());
    }
}

impl AwsSesMailer {
    fn build_destination(m: &Message) -> Destination {
        let mut builder = Destination::builder();

        let to = m.to.iter().map(|m| m.to_string()).collect();
        builder = builder.set_to_addresses(Some(to));

        if !m.cc.is_empty() {
            let cc = m.cc.iter().map(|m| m.to_string()).collect();
            builder = builder.set_cc_addresses(Some(cc));
        }

        if !m.bcc.is_empty() {
            let bcc = m.bcc.iter().map(|m| m.to_string()).collect();
            builder = builder.set_bcc_addresses(Some(bcc));
        }

        return builder.build();
    }

    fn build_content(m: &Message) -> EmailContent {
        let mut builder = SesMessage::builder();

        if !m.headers.is_empty() {
            let headers = m.headers.iter().map(|(k, v)| build_header(k, v)).collect();
            builder = builder.set_headers(Some(headers));
        }

        let subject = encode_string(&m.subject);
        let body = Self::build_body(m);
        builder = builder.subject(subject).body(body);

        return EmailContent::builder().simple(builder.build()).build();
    }

    fn build_body(m: &Message) -> Body {
        let mut arnold = Body::builder();

        if let Some(body) = &m.text_body {
            arnold = arnold.text(encode_string(body));
        }

        if let Some(body) = &m.html_body {
            arnold = arnold.html(encode_string(body));
        }

        return arnold.build();
    }
}

fn build_tag(k: &str, v: &str) -> MessageTag {
    return MessageTag::builder()
        .name(k)
        .value(v)
        .build()
        .expect("Name and value should be set");
}

fn build_header(k: &str, v: &str) -> MessageHeader {
    return MessageHeader::builder()
        .name(k)
        .value(v)
        .build()
        .expect("Name and value should be set");
}

fn encode_string(s: &str) -> Content {
    return Content::builder()
        .charset("UTF-8")
        .data(s)
        .build()
        .expect("Data should be set");
}
