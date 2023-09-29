#![warn(unreachable_pub)]

use std::str::FromStr;

use async_trait::async_trait;
use email_address::EmailAddress;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport, Transport};
use repaint_server_usecase::infra::email::EmailSender;
use teloc::dev::DependencyClone;

#[derive(Clone)]
pub struct Gmail {
    client: SmtpTransport,
    send_from: EmailAddress,
    url: String,
}

impl Gmail {
    pub fn new(send_from: String, url: String, username: String, password: String) -> Self {
        let send_from = EmailAddress::from_str(send_from.as_str()).expect("invalid email address");
        let cred = Credentials::new(username, password);
        let client = SmtpTransport::relay("smtp.gmail.com")
            .expect("failed to relay smtp.gmail.com")
            .credentials(cred)
            .build();

        Self {
            client,
            send_from,
            url,
        }
    }
}

impl DependencyClone for Gmail {}

#[async_trait]
impl EmailSender for Gmail {
    type Error = Error;

    async fn send(&self, to: EmailAddress, token: String) -> Result<(), Self::Error> {
        let mail = Message::builder()
            .from(self.send_from.to_string().parse().unwrap())
            .to(to.to_string().parse().unwrap())
            .subject("【Re:paint】管理者への追加のお知らせ")
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                include_str!("./message.tmp.txt"),
                URL = self.url,
                TOKEN = token
            ))
            .expect("failed to build message");
        self.client.send(&mail)?;

        Ok(())
    }
}
