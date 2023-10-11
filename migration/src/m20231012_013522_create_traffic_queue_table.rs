use sea_orm_migration::prelude::*;

use crate::m20230712_213646_create_event_spot_table::EventSpots;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_traffic_queue() -> TableCreateStatement {
        #[rustfmt::skip]
        let traffic_queue = Table::create()
            .table(TrafficQueues::Table)
            .if_not_exists()
            .col(ColumnDef::new(TrafficQueues::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(TrafficQueues::SpotId).integer().not_null())
            .col(ColumnDef::new(TrafficQueues::HeadCountFrom).char_len(26).not_null())
            .col(ColumnDef::new(TrafficQueues::HeadCountTo).char_len(26).not_null())
            .col(ColumnDef::new(TrafficQueues::Timestamp).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).date_time().not_null())
            .col(ColumnDef::new(TrafficQueues::CreatedAt).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).date_time().not_null())
            .col(ColumnDef::new(TrafficQueues::UpdatedAt).date_time())
            .foreign_key(foreign_key!(TrafficQueues::SpotId to EventSpots::Id Cascade))
            .to_owned();

        traffic_queue
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let traffic_queue = Self::up_traffic_queue();

        manager.create_table(traffic_queue).await?;

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
            TrafficQueues
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TrafficQueues {
    Table,
    Id,
    SpotId,
    HeadCountFrom,
    HeadCountTo,
    Timestamp,
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
    fn test_traffic_queue_table() {
        let traffic_queue = Migration::up_traffic_queue();

        self::assert_eq!(
            traffic_queue.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "traffic_queues" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""spot_id" integer NOT NULL,"#,
                r#""head_count_from" char(26) NOT NULL,"#,
                r#""head_count_to" char(26) NOT NULL,"#,
                r#""timestamp" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("spot_id") REFERENCES "event_spots" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#,
            ].join(" ")
        )
    }
}
