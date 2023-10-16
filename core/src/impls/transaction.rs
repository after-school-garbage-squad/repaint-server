use async_trait::async_trait;
use repaint_server_usecase::infra::repo::TransactionRepository;
use sea_orm::{DatabaseTransaction, TransactionTrait};

use crate::{Error, SeaOrm};

#[async_trait]
impl TransactionRepository for SeaOrm {
    type Error = Error;

    async fn begin_transaction(&self) -> Result<DatabaseTransaction, Self::Error> {
        self.con().begin().await.map_err(Into::into)
    }
}
