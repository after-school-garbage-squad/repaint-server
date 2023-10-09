use async_trait::async_trait;
use repaint_server_model::event::{Contact, Event};
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{EventRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};

use crate::entity::{admins, events, events_admins};
use crate::ty::json::AsJson;
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: events::Model) -> Result<Event, Error> {
    Ok(Event {
        id: m.id,
        event_id: m.event_id.model(),
        name: m.name,
        hp_url: m.hp_url,
        contact: Contact {
            name: m.contact.clone().model().name,
            email: m.contact.clone().model().email,
            phone: m.contact.model().phone,
        },
    })
}

#[async_trait]
impl EventRepository for SeaOrm {
    type Error = Error;

    async fn get_event_belong_to_subject(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Option<Event>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject))
            .find_with_related(events::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, events)| events)
            .flatten()
            .find(|e| e.event_id == event_id.dty())
            .map(to_model)
            .transpose()
    }

    async fn create(
        &self,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<Event, Self::Error> {
        let event = events::ActiveModel {
            event_id: Set(Id::new().dty()),
            name: Set(name),
            hp_url: Set(hp_url),
            contact: Set(AsJson(contact)),
            ..Default::default()
        }
        .insert(self.con())
        .await?;

        to_model(event)
    }

    async fn delete(&self, event_id: i32) -> Result<IsUpdated, Self::Error> {
        events_admins::Entity::delete_many()
            .filter(events_admins::Column::EventId.eq(event_id))
            .exec(self.con())
            .await
            .to_is_updated()
    }

    async fn list(&self, subject: String) -> Result<Vec<Event>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject.dty()))
            .find_with_related(events::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, events)| events)
            .flatten()
            .map(to_model)
            .collect()
    }

    async fn update(
        &self,
        event_id: i32,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<Option<Event>, Self::Error> {
        let tx = self.con().begin().await?;

        let mut event: events::ActiveModel =
            match events::Entity::find_by_id(event_id).one(&tx).await? {
                Some(m) => m.into(),
                None => return Ok(None),
            };
        event.name = Set(name);
        event.hp_url = Set(hp_url);
        event.contact = Set(AsJson(contact));
        let event = event.update(&tx).await?;
        tx.commit().await?;

        to_model(event).map(Some)
    }

    async fn get(&self, event_id: Id<Event>) -> Result<Option<Event>, Self::Error> {
        events::Entity::find()
            .filter(events::Column::EventId.eq(event_id.dty()))
            .one(self.con())
            .await?
            .map(to_model)
            .transpose()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::str::FromStr;

    use email_address::EmailAddress;
    use pretty_assertions::*;

    use crate::TestingSeaOrm;

    use super::*;

    impl TestingSeaOrm {
        pub(crate) async fn make_test_event(&self) -> Event {
            let event = crate::entity::events::ActiveModel {
                event_id: Set(Id::new().dty()),
                name: Set("test".into()),
                hp_url: Set("https://example.com".into()),
                contact: Set(AsJson(Contact {
                    name: "test".into(),
                    email: EmailAddress::from_str("test@example.com").unwrap(),
                    phone: "0120-10-7929".into(),
                })),
                ..Default::default()
            }
            .insert(self.orm().con())
            .await
            .unwrap();

            to_model(event).unwrap()
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_get_event_belong_to_subject() {
        let orm = TestingSeaOrm::new().await;
        let admin = orm.make_test_admin().await;
        let event = orm.make_test_event().await;
        let _ = repaint_server_usecase::infra::repo::AdminRepository::update(
            orm.orm(),
            admin.id,
            event.id,
        )
        .await
        .unwrap();

        let res =
            EventRepository::get_event_belong_to_subject(orm.orm(), admin.subject, event.event_id)
                .await
                .unwrap()
                .unwrap();

        self::assert_eq!(res, event);
    }

    #[test_log::test(tokio::test)]
    async fn test_list() {
        async fn test(q: u8) {
            let orm = TestingSeaOrm::new().await;
            let admin = orm.make_test_admin().await;
            let mut events = Vec::new();
            for _ in 0..q {
                let e = orm.make_test_event().await;
                let _ = repaint_server_usecase::infra::repo::AdminRepository::update(
                    orm.orm(),
                    admin.id,
                    e.id,
                )
                .await
                .unwrap();
                events.push(e);
            }

            let res = EventRepository::list(orm.orm(), admin.subject.clone())
                .await
                .unwrap();

            self::assert_eq!(res, events);
        }

        test(1).await;
        test(2).await;
        test(3).await;
    }

    #[test_log::test(tokio::test)]
    async fn test_get() {
        let orm = TestingSeaOrm::new().await;
        let admin = orm.make_test_admin().await;
        let mut events = Vec::new();
        for _ in 0..3 {
            let e = orm.make_test_event().await;
            let _ = repaint_server_usecase::infra::repo::AdminRepository::update(
                orm.orm(),
                admin.id,
                e.id,
            )
            .await
            .unwrap();
            events.push(e);
        }

        let res = EventRepository::get(orm.orm(), events[1].event_id)
            .await
            .unwrap()
            .unwrap();

        self::assert_eq!(res, events[1]);
    }

    #[test_log::test(tokio::test)]
    async fn test_update() {
        let orm = TestingSeaOrm::new().await;
        let admin = orm.make_test_admin().await;
        let event = orm.make_test_event().await;
        let _ = repaint_server_usecase::infra::repo::AdminRepository::update(
            orm.orm(),
            admin.id,
            event.id,
        )
        .await
        .unwrap();
        let res = EventRepository::update(
            orm.orm(),
            event.id,
            "test2".into(),
            event.hp_url.clone(),
            Contact {
                name: "test2".into(),
                email: event.contact.email.clone(),
                phone: event.contact.phone.clone(),
            },
        )
        .await
        .unwrap();

        self::assert_eq!(
            res,
            Some(Event {
                id: event.id,
                event_id: event.event_id,
                name: "test2".into(),
                hp_url: event.hp_url,
                contact: Contact {
                    name: "test2".into(),
                    email: event.contact.email,
                    phone: event.contact.phone,
                },
            })
        );
    }

    #[test_log::test(tokio::test)]
    async fn test_delete() {
        let orm = TestingSeaOrm::new().await;
        let admin = orm.make_test_admin().await;
        let event = orm.make_test_event().await;
        let mut events = Vec::new();
        let mut admins = Vec::new();
        for _ in 0..3 {
            let e = orm.make_test_event().await;
            let _ = repaint_server_usecase::infra::repo::AdminRepository::update(
                orm.orm(),
                admin.id,
                e.id,
            )
            .await
            .unwrap();
            events.push(e);
        }
        for _ in 0..3 {
            let a = orm.make_test_admin().await;
            let _ = repaint_server_usecase::infra::repo::AdminRepository::update(
                orm.orm(),
                a.id,
                event.id,
            )
            .await
            .unwrap();
            admins.push(a);
        }
        events.push(event.clone());
        let _ = EventRepository::delete(orm.orm(), events[1].id)
            .await
            .unwrap();
        events.remove(1);
        let res1 = EventRepository::list(orm.orm(), admin.subject.clone())
            .await
            .unwrap();
        let res2 = EventRepository::list(orm.orm(), admins[0].subject.clone())
            .await
            .unwrap();
        let _ = EventRepository::delete(orm.orm(), event.id).await.unwrap();
        let res3 = EventRepository::list(orm.orm(), admin.subject.clone())
            .await
            .unwrap();
        events.remove(2);

        self::assert_eq!(res1, events);
        self::assert_eq!(res2, [event]);
        self::assert_eq!(res3, events);
    }
}
