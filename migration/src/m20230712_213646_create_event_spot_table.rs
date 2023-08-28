use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Events;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_event_spot() -> TableCreateStatement {
        #[rustfmt::skip]
        let event_spot = Table::create()
            .table(EventSpots::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventSpots::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventSpots::EventID).integer().not_null())
            .col(ColumnDef::new(EventSpots::SpotId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(EventSpots::Name).string_len(32).not_null())
            .col(ColumnDef::new(EventSpots::IsPick).boolean().not_null().default(false))
            .col(ColumnDef::new(EventSpots::Bonus).boolean().not_null().default(false))
            .col(ColumnDef::new(EventSpots::Major).small_integer().not_null())
            .col(ColumnDef::new(EventSpots::Minor).small_integer().not_null())
            .col(ColumnDef::new(EventSpots::BeaconUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventSpots::HwId).char_len(10).not_null().unique_key())
            .col(ColumnDef::new(EventSpots::ServiceUuid).char_len(16).not_null())
            .col(ColumnDef::new(EventSpots::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(EventSpots::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventSpots::EventID to Events::Id Cascade))
            .to_owned();

        event_spot
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let event_spot = Self::up_event_spot();

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
            EventSpots,
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum EventSpots {
    Table,
    Id,
    EventID,
    SpotId,
    Name,
    IsPick,
    Bonus,
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
    fn test_event_spot_table() {
        let event_spot = Migration::up_event_spot();

        assert_eq!(
            event_spot.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "event_spots" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" integer NOT NULL,"#,
                r#""spot_id" char(26) NOT NULL UNIQUE,"#,
                r#""name" varchar(32) NOT NULL,"#,
                r#""is_pick" bool NOT NULL DEFAULT FALSE,"#,
                r#""bonus" bool NOT NULL DEFAULT FALSE,"#,
                r#""major" smallint NOT NULL,"#,
                r#""minor" smallint NOT NULL,"#,
                r#""beacon_uuid" char(16) NOT NULL,"#,
                r#""hw_id" char(10) NOT NULL UNIQUE,"#,
                r#""service_uuid" char(16) NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "events" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        );
    }
}
