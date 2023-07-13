use sea_orm_migration::prelude::*;

use crate::m20230712_213646_create_event_spot_table::EventSpot;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let event_spot = Table::create()
            .table(EventBeacon::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventBeacon::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventBeacon::SpotId).integer().not_null())
            .col(ColumnDef::new(EventBeacon::Major).integer().not_null())
            .col(ColumnDef::new(EventBeacon::Minor).integer().not_null())
            .col(ColumnDef::new(EventBeacon::BeaconUuid).string().not_null())
            .col(ColumnDef::new(EventBeacon::HwId).string().not_null())
            .col(ColumnDef::new(EventBeacon::ServiceUuid).string())
            .col(ColumnDef::new(EventBeacon::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(EventBeacon::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventBeacon::SpotId to EventSpot::Id Cascade))
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
            EventBeacon,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum EventBeacon {
    Table,
    Id,
    SpotId,
    Major,
    Minor,
    BeaconUuid,
    HwId,
    ServiceUuid,
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
