use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let event = Table::create()
            .table(Event::Table)
            .if_not_exists()
            .col(ColumnDef::new(Event::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Event::EventId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(Event::Name).string_len(32).not_null())
            .col(ColumnDef::new(Event::HpUrl).string_len(2083).not_null())
            .col(ColumnDef::new(Event::Contact).string_len(126).not_null())
            .col(ColumnDef::new(Event::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(Event::UpdatedAt).date_time())
            .to_owned();

        manager.create_table(event).await?;

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
            Event,
        }

        Ok(())
    }
}

#[derive(Iden)]
pub enum Event {
    Table,
    Id,
    EventId,
    Name,
    HpUrl,
    Contact,
    CreatedAt,
    UpdatedAt,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sea_orm_migration::prelude::*;

    use super::Event;

    #[test]
    fn test_event_table() {
        #[rustfmt::skip]
        let event = Table::create()
            .table(Event::Table)
            .if_not_exists()
            .col(ColumnDef::new(Event::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Event::EventId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(Event::Name).string_len(32).not_null())
            .col(ColumnDef::new(Event::HpUrl).string_len(2083).not_null())
            .col(ColumnDef::new(Event::Contact).string_len(126).not_null())
            .col(ColumnDef::new(Event::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(Event::UpdatedAt).date_time())
            .to_owned();

        assert_eq!(
            event.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "event" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" char(26) NOT NULL UNIQUE,"#,
                r#""name" varchar(32) NOT NULL,"#,
                r#""hp_url" varchar(2083) NOT NULL,"#,
                r#""contact" varchar(126) NOT NULL,"#,
                r#""created_at" timestamp without time zone NOT NULL,"#,
                r#""updated_at" timestamp without time zone"#,
                r#")"#,
            ]
            .join(" ")
        );
    }
}
