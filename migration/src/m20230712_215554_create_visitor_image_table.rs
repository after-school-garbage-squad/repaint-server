use sea_orm_migration::prelude::*;

use crate::m20230712_215023_create_visitor_table::Visitors;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_visitor_image() -> TableCreateStatement {
        #[rustfmt::skip]
        let visitor_image = Table::create()
            .table(VisitorImages::Table)
            .if_not_exists()
            .col(ColumnDef::new(VisitorImages::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(VisitorImages::VisitorId).integer().not_null())
            .col(ColumnDef::new(VisitorImages::ImageId).char_len(26).not_null())
            .col(ColumnDef::new(VisitorImages::CurrentImageID).char_len(26).not_null())
            .col(ColumnDef::new(VisitorImages::CreatedAt).date_time().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
            .col(ColumnDef::new(VisitorImages::UpdatedAt).date_time())
            .foreign_key(foreign_key!(VisitorImages::VisitorId to Visitors::Id Cascade))
            .to_owned();

        visitor_image
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let visitor_image = Self::up_visitor_image();

        manager.create_table(visitor_image).await?;

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
            VisitorImages,
        }

        Ok(())
    }
}

#[derive(Iden)]
enum VisitorImages {
    Table,
    Id,
    VisitorId,
    ImageId,
    CurrentImageID,
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
    fn test_visitor_image_tanle() {
        let visitor_image = Migration::up_visitor_image();

        assert_eq!(
            visitor_image.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "visitor_images" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""visitor_id" integer NOT NULL,"#,
                r#""image_id" char(26) NOT NULL,"#,
                r#""current_image_id" char(26) NOT NULL,"#,
                r#""created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("visitor_id") REFERENCES "visitors" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        );
    }
}
