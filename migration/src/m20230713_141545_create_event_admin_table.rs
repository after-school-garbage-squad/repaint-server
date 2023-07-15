use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Event;
use crate::m20230712_222322_create_admin_table::Admin;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let event_admin = Table::create()
            .table(EventAdmin::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventAdmin::AdminId).integer().not_null())
            .col(ColumnDef::new(EventAdmin::EventId).integer().not_null())
            .foreign_key(foreign_key!(EventAdmin::AdminId to Admin::Id Restrict))
            .foreign_key(foreign_key!(EventAdmin::EventId to Event::Id Restrict))
            .to_owned();

        manager.create_table(event_admin).await?;

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
            EventAdmin,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum EventAdmin {
    Table,
    AdminId,
    EventId,
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

    use super::Admin;
    use super::Event;
    use super::EventAdmin;

    #[test]
    fn test_event_admin_table() {
        #[rustfmt::skip]
        let event_admin = Table::create()
            .table(EventAdmin::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventAdmin::AdminId).integer().not_null())
            .col(ColumnDef::new(EventAdmin::EventId).integer().not_null())
            .foreign_key(foreign_key!(EventAdmin::AdminId to Admin::Id Restrict))
            .foreign_key(foreign_key!(EventAdmin::EventId to Event::Id Restrict))
            .to_owned();

        assert_eq!(
            event_admin.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "event_admin" ("#,
                r#""admin_id" integer NOT NULL,"#,
                r#""event_id" integer NOT NULL,"#,
                r#"FOREIGN KEY ("admin_id") REFERENCES "admin" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "event" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT"#,
                r#")"#
            ].join(" ")
        )
    }
}
