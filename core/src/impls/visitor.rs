use async_trait::async_trait;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::infra::repo::{IsUpdated, VisitorRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::entity::{events, visitors};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: visitors::Model) -> Result<Visitor, Error> {
    Ok(Visitor {
        id: m.id,
        visitor_id: m.visitor_id.model(),
        registration_id: m.registration_id,
    })
}

#[async_trait]
impl VisitorRepository for SeaOrm {
    type Error = Error;

    async fn create(&self, event_id: i32, registration_id: String) -> Result<Visitor, Self::Error> {
        let visitor = visitors::ActiveModel {
            event_id: Set(event_id),
            visitor_id: Set(Id::new().dty()),
            registration_id: Set(registration_id),
            ..Default::default()
        }
        .insert(self.con())
        .await?;

        to_model(visitor)
    }

    async fn get(
        &self,
        event_id: i32,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Visitor>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(visitors::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, v)| v)
            .flatten()
            .find(|v| v.visitor_id == visitor_id.dty())
            .map(to_model)
            .transpose()
    }

    async fn delete(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error> {
        visitors::Entity::delete_by_id(visitor_id)
            .exec(self.con())
            .await
            .to_is_updated()
    }

    async fn list(&self, event_id: i32) -> Result<Vec<Visitor>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(visitors::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, v)| v)
            .flatten()
            .map(to_model)
            .collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::*;

    use crate::TestingSeaOrm;

    use super::*;

    impl TestingSeaOrm {
        pub(crate) async fn make_test_visitor(&self, event_id: i32) -> Visitor {
            let visitor = visitors::ActiveModel {
                event_id: Set(event_id),
                visitor_id: Set(Id::new().dty()),
                registration_id: Set("testtesttest".into()),
                ..Default::default()
            }
            .insert(self.orm().con())
            .await
            .unwrap();

            to_model(visitor).unwrap()
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_get() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let visitor = orm.make_test_visitor(event.id).await;

        let res = VisitorRepository::get(orm.orm(), event.id, visitor.visitor_id)
            .await
            .unwrap();

        self::assert_eq!(res, Some(visitor));
    }

    #[test_log::test(tokio::test)]
    async fn test_list() {
        async fn test(q: u8) {
            let orm = TestingSeaOrm::new().await;
            let event = orm.make_test_event().await;
            let mut visitors = Vec::<Visitor>::new();
            for _ in 0..q {
                visitors.push(orm.make_test_visitor(event.id).await);
            }

            let res = VisitorRepository::list(orm.orm(), event.id).await.unwrap();

            self::assert_eq!(res, visitors);
        }

        test(1).await;
        test(2).await;
        test(3).await;
    }

    #[test_log::test(tokio::test)]
    async fn test_delete() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let mut visitors = Vec::<Visitor>::new();
        for _ in 0..3 {
            visitors.push(orm.make_test_visitor(event.id).await);
        }
        let _ = VisitorRepository::delete(orm.orm(), visitors[1].id)
            .await
            .unwrap();
        visitors.remove(1);

        let res = VisitorRepository::list(orm.orm(), event.id).await.unwrap();

        self::assert_eq!(res, visitors);
    }
}
