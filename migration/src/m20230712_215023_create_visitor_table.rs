use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Event;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_visitor() -> TableCreateStatement {
        #[rustfmt::skip]
        let visitor = Table::create()
            .table(Visitor::Table)
            .if_not_exists()
            .col(ColumnDef::new(Visitor::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Visitor::EventId).integer().not_null())
            .col(ColumnDef::new(Visitor::VisitorId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(Visitor::RegistrationId).char_len(16).not_null())
            .col(ColumnDef::new(Visitor::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(Visitor::UpdatedAt).date_time())
            .foreign_key(foreign_key!(Visitor::EventId to Event::Id Cascade))
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
            Visitor,
        }

        Ok(())
    }
}

#[derive(Iden)]
pub enum Visitor {
    Table,
    Id,
    EventId,
    VisitorId,
    RegistrationId,
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
    fn test_visitor_table() {
        let visitor = Migration::up_visitor();

        assert_eq!(
            visitor.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "visitor" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""event_id" integer NOT NULL,"#,
                r#""visitor_id" char(26) NOT NULL UNIQUE,"#,
                r#""registration_id" char(16) NOT NULL,"#,
                r#""created_at" timestamp without time zone NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "event" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        );
    }
}
