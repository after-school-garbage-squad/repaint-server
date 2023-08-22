use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_admin() -> TableCreateStatement {
        #[rustfmt::skip]
        let admin = Table::create()
            .table(Admins::Table)
            .if_not_exists()
            .col(ColumnDef::new(Admins::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Admins::AdminId).char_len(26).unique_key())
            .col(ColumnDef::new(Admins::Subject).string_len(64).not_null().unique_key())
            .col(ColumnDef::new(Admins::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(Admins::UpdatedAt).date_time())
            .to_owned();

        admin
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let admin = Self::up_admin();

        manager.create_table(admin).await?;

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
            Admins,
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Admins {
    Table,
    Id,
    AdminId,
    Subject,
    CreatedAt,
    UpdatedAt,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sea_orm_migration::prelude::*;

    use super::Migration;

    #[test]
    fn test_admin_table() {
        #[rustfmt::skip]
        let admin = Migration::up_admin();

        assert_eq!(
            admin.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "admins" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""admin_id" char(26) UNIQUE,"#,
                r#""subject" varchar(64) NOT NULL UNIQUE,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone"#,
                r#")"#
            ]
            .join(" ")
        )
    }
}
