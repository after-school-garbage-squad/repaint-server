use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Event;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let event_spot = Table::create()
            .table(EventSpot::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventSpot::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventSpot::EventID).integer().not_null())
            .col(ColumnDef::new(EventSpot::SpotId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(EventSpot::Name).string_len(32).not_null())
            .col(ColumnDef::new(EventSpot::IsPick).boolean().not_null().default(false))
            .col(ColumnDef::new(EventSpot::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(EventSpot::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventSpot::EventID to Event::Id Cascade))
            .to_owned();

        manager.create_table(event_spot).await?;

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
            EventSpot,
        }

        Ok(())
    }
}

#[derive(Iden)]
pub enum EventSpot {
    Table,
    Id,
    EventID,
    SpotId,
    Name,
    IsPick,
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
