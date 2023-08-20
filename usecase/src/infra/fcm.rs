use async_trait::async_trait;
use repaint_server_model::{AsyncSafe, StaticError};

use crate::model::fcm::Topic;

#[async_trait]
pub trait FirebaseCloudMessaging: AsyncSafe {
    type Error: StaticError;

    async fn send(
        &self,
        registration_id: String,
        topic: Topic,
        message: String,
    ) -> Result<(), Self::Error>;
}
