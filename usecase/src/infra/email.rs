use async_trait::async_trait;
use email_address::EmailAddress;
use repaint_server_model::{AsyncSafe, StaticError};

#[async_trait]
pub trait EmailSender: AsyncSafe {
    type Error: StaticError;

    async fn send(&self, to: EmailAddress, token: String) -> Result<(), Self::Error>;
}
