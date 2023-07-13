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
            .col(ColumnDef::new(Event::EventId).string().not_null().unique_key())
            .col(ColumnDef::new(Event::Name).string().not_null())
            .col(ColumnDef::new(Event::HpUrl).string().not_null())
            .col(ColumnDef::new(Event::Contact).json().not_null())
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
