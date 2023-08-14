use sea_orm_migration::prelude::*;

use crate::m20230712_213646_create_event_spot_table::EventSpots;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_event_beacon() -> TableCreateStatement {
        #[rustfmt::skip]
        let event_beacon = Table::create()
            .table(EventBeacons::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventBeacons::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventBeacons::SpotId).integer().not_null())
            .col(ColumnDef::new(EventBeacons::Major).small_integer().not_null())
            .col(ColumnDef::new(EventBeacons::Minor).small_integer().not_null())
            .col(ColumnDef::new(EventBeacons::BeaconUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventBeacons::HwId).string_len(10).unique_key())
            .col(ColumnDef::new(EventBeacons::ServiceUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventBeacons::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(EventBeacons::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventBeacons::SpotId to EventSpots::Id Cascade))
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
            EventBeacons,
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EventBeacons {
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
                r#"CREATE TABLE IF NOT EXISTS "event_beacons" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""spot_id" integer NOT NULL,"#,
                r#""major" smallint NOT NULL,"#,
                r#""minor" smallint NOT NULL,"#,
                r#""beacon_uuid" char(16) NOT NULL,"#,
                r#""hw_id" varchar(10) UNIQUE,"#,
                r#""service_uuid" char(16) NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("spot_id") REFERENCES "event_spots" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        )
    }
}
