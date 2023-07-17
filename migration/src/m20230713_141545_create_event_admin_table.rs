use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(
            manager.get_database_backend(),
            [
                r#"CREATE TABLE IF NOT EXISTS "events_admins" ("#,
                r#""admin_id" integer NOT NULL,"#,
                r#""event_id" integer NOT NULL,"#,
                r#"PRIMARY KEY ("admin_id", "event_id"),"#,
                r#"FOREIGN KEY ("admin_id") REFERENCES "admins" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT,"#,
                r#"FOREIGN KEY ("event_id") REFERENCES "events" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT"#,
                r#")"#
            ].join(" ").to_string()
        )).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE 'events_admins'")
            .await?;

        Ok(())
    }
}
