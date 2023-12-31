//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "traffic_queues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub spot_id: i32,
    pub head_count_from: i32,
    pub head_count_to: i32,
    pub timestamp: DateTime,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::event_spots::Entity",
        from = "Column::SpotId",
        to = "super::event_spots::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    EventSpots,
}

impl Related<super::event_spots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventSpots.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
