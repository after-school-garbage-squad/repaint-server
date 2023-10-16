use async_trait::async_trait;
use repaint_server_model::admin::Admin;
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{AdminRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

use crate::entity::{admins, events_admins};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: admins::Model) -> Result<Admin, Error> {
    Ok(Admin {
        id: m.id,
        admin_id: m.admin_id.model(),
        subject: m.subject,
    })
}

#[async_trait]
impl AdminRepository for SeaOrm {
    type Error = Error;

    async fn add(&self, subject: String) -> Result<IsUpdated, Self::Error> {
        admins::ActiveModel {
            subject: Set(subject),
            admin_id: Set(Id::new().dty()),
            ..Default::default()
        }
        .insert(self.con())
        .await
        .to_is_updated()
    }

    async fn get(&self, subject: String) -> Result<Option<Admin>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject))
            .one(self.con())
            .await?
            .map(to_model)
            .transpose()
    }

    async fn get_with_tx(
        &self,
        tx: &DatabaseTransaction,
        subject: String,
    ) -> Result<Option<Admin>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject))
            .one(tx)
            .await?
            .map(to_model)
            .transpose()
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        admin_id: i32,
        event_id: i32,
    ) -> Result<IsUpdated, Self::Error> {
        events_admins::ActiveModel {
            event_id: Set(event_id),
            admin_id: Set(admin_id),
            ..Default::default()
        }
        .insert(tx)
        .await
        .to_is_updated()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::*;
    use rand::distributions::Alphanumeric;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use repaint_server_usecase::infra::repo::TransactionRepository;

    use crate::TestingSeaOrm;

    use super::*;

    impl TestingSeaOrm {
        pub(crate) async fn make_test_admin(&self) -> Admin {
            let rng = {
                let rng = rand::thread_rng();
                StdRng::from_rng(rng).unwrap()
            };
            let c = rng
                .sample_iter(&Alphanumeric)
                .take(23)
                .map(char::from)
                .collect::<String>();
            let admin = crate::entity::admins::ActiveModel {
                admin_id: Set(Id::new().dty()),
                subject: Set(format!("auth0|{}", c).into()),
                ..Default::default()
            }
            .insert(self.orm().con())
            .await
            .unwrap();

            to_model(admin).unwrap()
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_get() {
        let orm = TestingSeaOrm::new().await;
        let admin = orm.make_test_admin().await;
        let res = AdminRepository::get(orm.orm(), admin.subject.clone())
            .await
            .unwrap();

        self::assert_eq!(res, Some(admin));
    }

    #[test_log::test(tokio::test)]
    async fn test_get_with_tx() {
        let orm = TestingSeaOrm::new().await;
        let admin = orm.make_test_admin().await;
        let tx = orm.orm().begin_transaction().await.unwrap();
        let res = AdminRepository::get_with_tx(orm.orm(), &tx, admin.subject.clone())
            .await
            .unwrap();
        let _ = tx.commit().await.unwrap();

        self::assert_eq!(res, Some(admin));
    }
}
