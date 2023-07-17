//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "visitor_palettes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub visitor_id: i32,
    pub palette_id_list: String,
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
