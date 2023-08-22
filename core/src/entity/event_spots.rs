//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use sea_orm::entity::prelude::*;

use crate::ty::string::AsString;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "event_spots")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub event_id: i32,
    #[sea_orm(unique)]
    pub spot_id: AsString<Id<EventSpot>>,
    pub name: String,
    pub is_pick: bool,
    pub bonus: bool,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::event_beacons::Entity")]
    EventBeacons,
    #[sea_orm(
        belongs_to = "super::events::Entity",
        from = "Column::EventId",
        to = "super::events::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Events,
}

impl Related<super::event_beacons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventBeacons.def()
    }
}

impl Related<super::events::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Events.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
