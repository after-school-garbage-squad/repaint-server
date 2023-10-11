//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image};
use sea_orm::entity::prelude::*;

use crate::ty::string::AsString;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "visitor_images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub visitor_id: i32,
    pub image_id: Option<AsString<Id<Image>>>,
    pub current_image_id: AsString<Id<CurrentImage>>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::visitors::Entity",
        from = "Column::VisitorId",
        to = "super::visitors::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Visitors,
}

impl Related<super::visitors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Visitors.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
