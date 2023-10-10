use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Events;
use crate::m20230712_213646_create_event_spot_table::EventSpots;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_visitor() -> TableCreateStatement {
        #[rustfmt::skip]
        let visitor = Table::create()
            .table(Visitors::Table)
            .if_not_exists()
            .col(ColumnDef::new(Visitors::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Visitors::EventId).integer().not_null())
            .col(ColumnDef::new(Visitors::VisitorId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(Visitors::RegistrationId).string_len(4096).not_null())
            .col(ColumnDef::new(Visitors::IsUpdated).boolean().not_null().default(false))
            .col(ColumnDef::new(Visitors::IsDownloadable).boolean().not_null().default(false))
            .col(ColumnDef::new(Visitors::LastScannedAt).date_time())
            .col(ColumnDef::new(Visitors::LastDropedAt).date_time())
            .col(ColumnDef::new(Visitors::LastScannedSpot).integer())
            .col(ColumnDef::new(Visitors::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(Visitors::UpdatedAt).date_time())
            .foreign_key(foreign_key!(Visitors::EventId to Events::Id Cascade))
            .foreign_key(foreign_key!(Visitors::LastScannedSpot to EventSpots::Id SetNull))
            .to_owned();

        visitor
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let visitor = Self::up_visitor();

        manager.create_table(visitor).await?;

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
            Visitors,
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Visitors {
    Table,
    Id,
    EventId,
    VisitorId,
    RegistrationId,
    IsUpdated,
    IsDownloadable,
    LastScannedAt,
    LastDropedAt,
    LastScannedSpot,
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
    use pretty_assertions::*;
    use sea_orm_migration::prelude::*;

    use super::Migration;

    #[test]
    fn test_visitor_table() {
        let visitor = Migration::up_visitor();

        self::assert_eq!(
            visitor.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "visitors" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" integer NOT NULL,"#,
                r#""visitor_id" char(26) NOT NULL UNIQUE,"#,
                r#""registration_id" varchar(4096) NOT NULL,"#,
                r#""is_updated" bool NOT NULL DEFAULT FALSE,"#,
                r#""is_downloadable" bool NOT NULL DEFAULT FALSE,"#,
                r#""last_scanned_at" timestamp without time zone,"#,
                r#""last_droped_at" timestamp without time zone,"#,
                r#""last_scanned_spot" integer,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "events" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
                r#"FOREIGN KEY ("last_scanned_spot") REFERENCES "event_spots" ("id") ON DELETE SET NULL ON UPDATE SET NULL"#,
                r#")"#
            ].join(" ")
        );
    }
}
