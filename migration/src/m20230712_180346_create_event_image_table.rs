use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Events;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_event_image() -> TableCreateStatement {
        #[rustfmt::skip]
        let events_images = Table::create()
            .table(EventsImages::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventsImages::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(EventsImages::EventId).integer().not_null())
            .col(ColumnDef::new(EventsImages::ImageID).char_len(16).not_null())
            .col(ColumnDef::new(EventsImages::CompressedImageID).char_len(16).not_null())
            .col(ColumnDef::new(EventsImages::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(EventsImages::UpdatedAt).date_time())
            .foreign_key(foreign_key!(EventsImages::EventId to Events::Id Cascade))
            .to_owned();

        events_images
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
            EventsImages,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum EventsImages {
    Table,
    Id,
    EventId,
    ImageID,
    CompressedImageID,
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
    fn test_event_image_table() {
        let event_image = Migration::up_event_image();

        assert_eq!(
            event_image.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "events_images" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" integer NOT NULL,"#,
                r#""image_id" char(16) NOT NULL,"#,
                r#""compressed_image_id" char(16) NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "events" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        )
    }
}
