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
