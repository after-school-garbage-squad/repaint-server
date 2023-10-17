use async_trait::async_trait;
use chrono::NaiveDateTime;
use repaint_server_model::admin::Admin;
use repaint_server_model::event::{Contact, Event};
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::{AsyncSafe, StaticError};
use sea_orm::DatabaseTransaction;

use crate::model::traffic::HeadCountResponse;

#[async_trait]
pub trait SpotRepository: AsyncSafe {
    type Error: StaticError;

    async fn register(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        name: String,
        hw_id: String,
        service_uuid: String,
    ) -> Result<EventSpot, Self::Error>;

    async fn list(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
    ) -> Result<Vec<EventSpot>, Self::Error>;

    async fn get_by_beacon(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        hw_id: String,
    ) -> Result<Option<EventSpot>, Self::Error>;

    async fn get_by_qr(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<EventSpot>, Self::Error>;

    async fn get_by_id(&self, spot_id: i32) -> Result<Option<EventSpot>, Self::Error>;

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        spot_id: Id<EventSpot>,
        name: String,
        is_pick: bool,
    ) -> Result<Option<EventSpot>, Self::Error>;

    async fn delete(
        &self,
        txn: &DatabaseTransaction,
        event_id: i32,
        spot_id: Id<EventSpot>,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_bonus_state(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        spot_id: Id<EventSpot>,
    ) -> Result<bool, Self::Error>;

    async fn set_bonus_state(
        &self,
        txn: &DatabaseTransaction,
        event_id: i32,
        spot_id: Id<EventSpot>,
        is_bonus: bool,
    ) -> Result<IsUpdated, Self::Error>;

    async fn scanned(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        spot_id: i32,
        now: NaiveDateTime,
    ) -> Result<IsUpdated, Self::Error>;
}

#[async_trait]
pub trait ImageRepository: AsyncSafe {
    type Error: StaticError;

    async fn add_default_image(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error>;

    async fn delete_default_image(
        &self,
        txn: &DatabaseTransaction,
        event_id: i32,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error>;

    async fn upload_visitor_image(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_visitor_image(
        &self,
        tx: &DatabaseTransaction,
        visitor_id: i32,
    ) -> Result<Option<Id<VisitorImage>>, Self::Error>;

    async fn list_default_image(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
    ) -> Result<Vec<Id<EventImage>>, Self::Error>;

    async fn get_current_image(
        &self,
        tx: &DatabaseTransaction,
        visitor_id: i32,
    ) -> Result<Option<Id<CurrentImage>>, Self::Error>;

    async fn set_current_image(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error>;
}

#[async_trait]
pub trait PaletteRepository: AsyncSafe {
    type Error: StaticError;

    async fn get(&self, tx: &DatabaseTransaction, visitor_id: i32)
        -> Result<Vec<i32>, Self::Error>;

    async fn set(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        palette: i32,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_all(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
    ) -> Result<Option<Vec<i32>>, Self::Error>;

    async fn set_all(
        &self,
        txn: &DatabaseTransaction,
        event_id: i32,
        palette: Vec<i32>,
    ) -> Result<IsUpdated, Self::Error>;
}

#[async_trait]
pub trait EventRepository: AsyncSafe {
    type Error: StaticError;

    async fn get_event_belong_to_subject(
        &self,
        tx: &DatabaseTransaction,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Option<Event>, Self::Error>;

    async fn create(
        &self,
        tx: &DatabaseTransaction,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<Event, Self::Error>;

    async fn delete(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
    ) -> Result<IsUpdated, Self::Error>;

    async fn list(
        &self,
        tx: &DatabaseTransaction,
        subject: String,
    ) -> Result<Vec<Event>, Self::Error>;

    async fn update(
        &self,
        txn: &DatabaseTransaction,
        event_id: i32,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<Option<Event>, Self::Error>;

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        event_id: Id<Event>,
    ) -> Result<Option<Event>, Self::Error>;
}

#[async_trait]
pub trait AdminRepository: AsyncSafe {
    type Error: StaticError;

    async fn add(&self, subject: String) -> Result<IsUpdated, Self::Error>;

    async fn get(&self, subject: String) -> Result<Option<Admin>, Self::Error>;

    async fn get_with_tx(
        &self,
        tx: &DatabaseTransaction,
        subject: String,
    ) -> Result<Option<Admin>, Self::Error>;

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        admin_id: i32,
        event_id: i32,
    ) -> Result<IsUpdated, Self::Error>;
}

#[async_trait]
pub trait VisitorRepository: AsyncSafe {
    type Error: StaticError;

    async fn create(&self, event_id: i32, registration_id: String) -> Result<Visitor, Self::Error>;

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Visitor>, Self::Error>;

    async fn get_by_id(&self, visitor_id: i32) -> Result<Option<Visitor>, Self::Error>;

    async fn delete(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error>;

    async fn list(&self, event_id: i32) -> Result<Vec<Visitor>, Self::Error>;

    async fn set_update(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
    ) -> Result<IsUpdated, Self::Error>;

    async fn unset_update(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
    ) -> Result<IsUpdated, Self::Error>;

    async fn check_update(&self, visitor_id: i32) -> Result<bool, Self::Error>;

    async fn set_last_droped_at(
        &self,
        tx: &DatabaseTransaction,
        visitor_id: i32,
        last_droped_at: NaiveDateTime,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_last_droped_at(
        &self,
        visitor_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error>;

    async fn set_last_picked_at(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        spot_id: i32,
        last_picked_at: NaiveDateTime,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_last_picked_at(
        &self,
        tx: &DatabaseTransaction,
        visitor_id: i32,
        spot_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error>;

    async fn get_last_scanned_at(
        &self,
        tx: &DatabaseTransaction,
        visitor_id: i32,
        spot_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error>;

    async fn get_visitors(
        &self,
        tx: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<Vec<i32>, Self::Error>;
}

#[async_trait]
pub trait TrafficRepository: AsyncSafe {
    type Error: StaticError;

    async fn size(&self) -> Result<usize, Self::Error>;

    async fn push(
        &self,
        spot_id: i32,
        hc_from: usize,
        hc_to: usize,
    ) -> Result<IsUpdated, Self::Error>;

    async fn pop(&self) -> Result<Option<i32>, Self::Error>;

    async fn remove(
        &self,
        txn: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_timestamp(
        &self,
        tx: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error>;

    async fn get_hc(
        &self,
        tx: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<Option<HeadCountResponse>, Self::Error>;
}

#[async_trait]
pub trait TransactionRepository: AsyncSafe {
    type Error: StaticError;

    async fn begin_transaction(&self) -> Result<DatabaseTransaction, Self::Error>;
}

#[derive(Debug)]
#[must_use]
pub struct IsUpdated(pub bool);

impl IsUpdated {
    pub fn ignore(self) {}

    pub fn ok_or<E>(self, e: E) -> Result<(), E> {
        self.ok_or_else(|| e)
    }

    pub fn ok_or_else<F, E>(self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> E,
    {
        if self.0 {
            Ok(())
        } else {
            Err(f())
        }
    }
}
