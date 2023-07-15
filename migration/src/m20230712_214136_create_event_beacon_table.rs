use sea_orm_migration::prelude::*;

use crate::m20230712_213646_create_event_spot_table::EventsSpots;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_event_beacon() -> TableCreateStatement {
        #[rustfmt::skip]
        let event_beacon = Table::create()
            .table(EventsBeacons::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventsBeacons::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventsBeacons::SpotId).integer().not_null())
            .col(ColumnDef::new(EventsBeacons::Major).small_integer().not_null())
            .col(ColumnDef::new(EventsBeacons::Minor).small_integer().not_null())
            .col(ColumnDef::new(EventsBeacons::BeaconUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventsBeacons::HwId).string_len(10).not_null())
            .col(ColumnDef::new(EventsBeacons::ServiceUuid).char_len(16))
            .col(ColumnDef::new(EventsBeacons::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(EventsBeacons::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventsBeacons::SpotId to EventsSpots::Id Cascade))
            .to_owned();

        event_beacon
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let event_beacon = Self::up_event_beacon();

        manager.create_table(event_beacon).await?;

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
            EventsBeacons,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum EventsBeacons {
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

    use super::Migration;

    #[test]
    fn test_event_beacon_table() {
        let event_spot = Migration::up_event_beacon();

        assert_eq!(
            event_spot.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "events_beacons" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""spot_id" integer NOT NULL,"#,
                r#""major" smallint NOT NULL,"#,
                r#""minor" smallint NOT NULL,"#,
                r#""beacon_uuid" char(16) NOT NULL,"#,
                r#""hw_id" varchar(10) NOT NULL,"#,
                r#""service_uuid" char(16),"#,
                r#""created_at" timestamp without time zone NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("spot_id") REFERENCES "events_spots" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        )
    }
}
