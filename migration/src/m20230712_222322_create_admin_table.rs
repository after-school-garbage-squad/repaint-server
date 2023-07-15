use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[rustfmt::skip]
        let admin = Table::create()
            .table(Admin::Table)
            .if_not_exists()
            .col(ColumnDef::new(Admin::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Admin::AdminId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(Admin::Email).string_len(80).not_null().unique_key())
            .col(ColumnDef::new(Admin::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(Admin::UpdatedAt).date_time())
            .to_owned();

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
            Admin,
        }

        Ok(())
    }
}

#[derive(Iden)]
pub enum Admin {
    Table,
    Id,
    AdminId,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sea_orm_migration::prelude::*;

    use super::Admin;

    #[test]
    fn test_admin_table() {
        #[rustfmt::skip]
        let admin = Table::create()
            .table(Admin::Table)
            .if_not_exists()
            .col(ColumnDef::new(Admin::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Admin::AdminId).char_len(26).not_null().unique_key())
            .col(ColumnDef::new(Admin::Email).string_len(80).not_null().unique_key())
            .col(ColumnDef::new(Admin::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(Admin::UpdatedAt).date_time())
            .to_owned();

        assert_eq!(
            admin.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "admin" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""admin_id" char(26) NOT NULL UNIQUE,"#,
                r#""email" varchar(80) NOT NULL UNIQUE,"#,
                r#""created_at" timestamp without time zone NOT NULL,"#,
                r#""updated_at" timestamp without time zone"#,
                r#")"#
            ]
            .join(" ")
        )
    }
}
