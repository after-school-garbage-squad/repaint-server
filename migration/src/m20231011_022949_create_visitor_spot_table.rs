use sea_orm_migration::prelude::*;

use crate::m20230712_213646_create_event_spot_table::EventSpots;
use crate::m20230712_215023_create_visitor_table::Visitors;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_visitor_spot() -> TableCreateStatement {
        #[rustfmt::skip]
        let visitor_spot = Table::create()
            .table(VisitorSpots::Table)
            .if_not_exists()
            .col(ColumnDef::new(VisitorSpots::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(VisitorSpots::VisitorId).integer().not_null())
            .col(ColumnDef::new(VisitorSpots::SpotId).integer().not_null())
            .col(ColumnDef::new(VisitorSpots::LastScannedAt).date_time().not_null())
            .col(ColumnDef::new(VisitorSpots::LastPickedAt).date_time())
            .col(ColumnDef::new(VisitorSpots::CreatedAt).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).date_time().not_null())
            .col(ColumnDef::new(VisitorSpots::UpdatedAt).date_time())
            .foreign_key(foreign_key!(VisitorSpots::VisitorId to Visitors::Id Cascade))
            .foreign_key(foreign_key!(VisitorSpots::SpotId to EventSpots::Id Cascade))
            .to_owned();

        visitor_spot
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let visitor_spot = Self::up_visitor_spot();

        manager.create_table(visitor_spot).await?;

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
            VisitorSpots
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum VisitorSpots {
    Table,
    Id,
    VisitorId,
    SpotId,
    LastScannedAt,
    LastPickedAt,
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
mod test {
    use pretty_assertions::*;
    use sea_orm_migration::prelude::*;

    use super::Migration;

    #[test]
    fn test_visitor_spot_table() {
        let visitor_spot = Migration::up_visitor_spot();

        self::assert_eq!(
            visitor_spot.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "visitor_spots" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""visitor_id" integer NOT NULL,"#,
                r#""spot_id" integer NOT NULL,"#,
                r#""last_scanned_at" timestamp without time zone NOT NULL,"#,
                r#""last_picked_at" timestamp without time zone,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("visitor_id") REFERENCES "visitors" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
                r#"FOREIGN KEY ("spot_id") REFERENCES "event_spots" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ]
            .join(" ")
        )
    }
}
