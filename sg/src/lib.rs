#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use email_address::EmailAddress;
use repaint_server_usecase::infra::email::EmailSender;
use sendgrid::error::SendgridError;
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};
use teloc::dev::DependencyClone;

#[derive(Debug, Clone)]
pub struct SendGrid {
    sender: Sender,
    send_from: EmailAddress,
    url: String,
}

impl SendGrid {
    pub fn new(api_key: &str, send_from: EmailAddress, url: &str) -> Self {
        let sender = Sender::new(api_key.to_string());

        Self {
            sender,
            send_from,
            url: url.to_string(),
        }
    }
}

impl DependencyClone for SendGrid {}

#[async_trait]
impl EmailSender for SendGrid {
    type Error = SendgridError;

    async fn send(&self, to: EmailAddress, token: String) -> Result<(), Self::Error> {
        let personalization = Personalization::new(Email::new(to.to_string()));
        let message = Message::new(Email::new(self.send_from.to_string()))
            .set_subject("【Re:paint】管理者への追加のお知らせ")
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(format!(
                        include_str!("./message.tmp.txt"),
                        URL = self.url,
                        TOKEN = token
                    )),
            )
            .add_personalization(personalization);
        self.sender.send(&message).await?;

        Ok(())
    }
}
