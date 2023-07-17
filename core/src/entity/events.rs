//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub event_id: Uuid,
    pub name: String,
    pub hp_url: String,
    pub contact: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::event_images::Entity")]
    EventImages,
    #[sea_orm(has_many = "super::event_spots::Entity")]
    EventSpots,
    #[sea_orm(has_many = "super::visitors::Entity")]
    Visitors,
}

impl Related<super::event_images::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventImages.def()
    }
}

impl Related<super::event_spots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventSpots.def()
    }
}

impl Related<super::visitors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Visitors.def()
    }
}

impl Related<super::admins::Entity> for Entity {
    fn to() -> RelationDef {
        super::events_admins::Relation::Admins.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::events_admins::Relation::Events.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
