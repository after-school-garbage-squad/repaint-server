use sea_orm_migration::prelude::*;

use crate::m20230712_215023_create_visitor_table::Visitor;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let visitor_image = Table::create()
            .table(VisitorImage::Table)
            .if_not_exists()
            .col(ColumnDef::new(VisitorImage::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(VisitorImage::VisitorId).integer().not_null())
            .col(ColumnDef::new(VisitorImage::ImageId).char_len(16).not_null())
            .col(ColumnDef::new(VisitorImage::CompressedImageID).char_len(16).not_null())
            .col(ColumnDef::new(VisitorImage::CurrentImageID).char_len(16).not_null())
            .col(ColumnDef::new(VisitorImage::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(VisitorImage::UpdatedAt).date_time())
            .foreign_key(foreign_key!(VisitorImage::VisitorId to Visitor::Id Cascade))
            .to_owned();

        manager.create_table(visitor_image).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        macro_rules! down {
            ($($name:ident),*$(,)?) => {
                $(manager
                    .drop_table(Table::drop().table($name::Table).to_owned())
                    .await?;)*
            };
        }

        down! {
            VisitorImage,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum VisitorImage {
    Table,
    Id,
    VisitorId,
    ImageId,
    CompressedImageID,
    CurrentImageID,
    CreatedAt,
    UpdatedAt,
}

macro_rules! foreign_key {
    ($from_table:ident::$from_col:ident to $to_table:ident::$to_col:ident $action:ident) => {
        ForeignKey::create()
            .from($from_table::Table, $from_table::$from_col)
            .to($to_table::Table, $to_table::$to_col)
            .on_delete(ForeignKeyAction::$action)
            .on_update(ForeignKeyAction::$action)
    };
}

use foreign_key;
