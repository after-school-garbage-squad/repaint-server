use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_events() -> TableCreateStatement {
        #[rustfmt::skip]
        let event = Table::create()
            .table(Events::Table)
            .if_not_exists()
            .col(ColumnDef::new(Events::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Events::EventId).char_len(26).unique_key().not_null())
            .col(ColumnDef::new(Events::Name).string_len(32).not_null())
            .col(ColumnDef::new(Events::HpUrl).string_len(2083).not_null())
            .col(ColumnDef::new(Events::Contact).json().not_null())
            .col(ColumnDef::new(Events::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(Events::UpdatedAt).date_time())
            .to_owned();

        event
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let event = Self::up_events();

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
            Events,
        }

        Ok(())
    }
}

#[derive(Iden)]
pub enum Events {
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

    use super::Migration;

    #[test]
    fn test_event_table() {
        #[rustfmt::skip]
        let events = Migration::up_events();

        assert_eq!(
            events.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "events" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" char(26) UNIQUE NOT NULL,"#,
                r#""name" varchar(32) NOT NULL,"#,
                r#""hp_url" varchar(2083) NOT NULL,"#,
                r#""contact" json NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone"#,
                r#")"#,
            ]
            .join(" ")
        );
    }
}
