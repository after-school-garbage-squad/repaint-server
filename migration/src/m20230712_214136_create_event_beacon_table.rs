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
            .col(ColumnDef::new(EventBeacon::Major).small_integer().not_null())
            .col(ColumnDef::new(EventBeacon::Minor).small_integer().not_null())
            .col(ColumnDef::new(EventBeacon::BeaconUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventBeacon::HwId).string_len(10).not_null())
            .col(ColumnDef::new(EventBeacon::ServiceUuid).char_len(16))
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sea_orm_migration::prelude::*;

    use super::EventBeacon;
    use super::EventSpot;

    #[test]
    fn test_event_beacon_table() {
        #[rustfmt::skip]
        let event_spot = Table::create()
            .table(EventBeacon::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventBeacon::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventBeacon::SpotId).integer().not_null())
            .col(ColumnDef::new(EventBeacon::Major).small_integer().not_null())
            .col(ColumnDef::new(EventBeacon::Minor).small_integer().not_null())
            .col(ColumnDef::new(EventBeacon::BeaconUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventBeacon::HwId).string_len(10).not_null())
            .col(ColumnDef::new(EventBeacon::ServiceUuid).char_len(16))
            .col(ColumnDef::new(EventBeacon::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(EventBeacon::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventBeacon::SpotId to EventSpot::Id Cascade))
            .to_owned();

        assert_eq!(
            event_spot.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "event_beacon" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""spot_id" integer NOT NULL,"#,
                r#""major" smallint NOT NULL,"#,
                r#""minor" smallint NOT NULL,"#,
                r#""beacon_uuid" char(16) NOT NULL,"#,
                r#""hw_id" varchar(10) NOT NULL,"#,
                r#""service_uuid" char(16),"#,
                r#""created_at" timestamp without time zone NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("spot_id") REFERENCES "event_spot" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        )
    }
}
