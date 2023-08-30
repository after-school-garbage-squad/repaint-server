use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Events;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_event_image() -> TableCreateStatement {
        #[rustfmt::skip]
        let event_image = Table::create()
            .table(EventImages::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventImages::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventImages::EventId).integer().not_null())
            .col(ColumnDef::new(EventImages::ImageID).char_len(26).unique_key().not_null())
            .col(ColumnDef::new(EventImages::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(EventImages::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventImages::EventId to Events::Id Cascade))
            .to_owned();

        event_image
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let event_image = Self::up_event_image();

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
            EventImages,
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EventImages {
    Table,
    Id,
    EventId,
    ImageID,
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
    fn test_event_image_table() {
        let event_image = Migration::up_event_image();

        self::assert_eq!(
            event_image.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "event_images" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" integer NOT NULL,"#,
                r#""image_id" char(26) UNIQUE NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "events" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        )
    }
}
