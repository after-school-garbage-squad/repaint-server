use sea_orm_migration::prelude::*;

use crate::m20230712_175819_create_event_table::Events;
use crate::m20230712_222322_create_admin_table::Admins;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_event_admin() -> TableCreateStatement {
        #[rustfmt::skip]
        let event_admin = Table::create()
            .table(EventsAdmins::Table)
            .if_not_exists()
            .col(ColumnDef::new(EventsAdmins::AdminId).integer().not_null())
            .col(ColumnDef::new(EventsAdmins::EventId).integer().not_null())
            .foreign_key(foreign_key!(EventsAdmins::AdminId to Admins::Id Restrict))
            .foreign_key(foreign_key!(EventsAdmins::EventId to Events::Id Restrict))
            .to_owned();

        event_admin
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let event_admin = Self::up_event_admin();

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
            EventsAdmins,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum EventsAdmins {
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

    use super::Migration;

    #[test]
    fn test_event_admin_table() {
        let event_admin = Migration::up_event_admin();

        assert_eq!(
            event_admin.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "events_admins" ("#,
                r#""admin_id" integer NOT NULL,"#,
                r#""event_id" integer NOT NULL,"#,
                r#"FOREIGN KEY ("admin_id") REFERENCES "admins" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "events" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT"#,
                r#")"#
            ].join(" ")
        )
    }
}
