use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Event;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let event_image = Table::create()
            .table(EventImage::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventImage::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventImage::EventId).integer().not_null())
            .col(ColumnDef::new(EventImage::ImageID).string().not_null())
            .col(ColumnDef::new(EventImage::CompressedImageID).string().not_null())
            .col(ColumnDef::new(EventImage::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(EventImage::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventImage::EventId to Event::Id Cascade))
            .to_owned();

        manager.create_table(event_image).await?;

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
            EventImage,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum EventImage {
    Table,
    Id,
    EventId,
    ImageID,
    CompressedImageID,
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
