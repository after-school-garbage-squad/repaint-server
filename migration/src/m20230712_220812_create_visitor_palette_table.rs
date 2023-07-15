use sea_orm_migration::prelude::*;

use crate::m20230712_215023_create_visitor_table::Visitors;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    fn up_visitor_palette() -> TableCreateStatement {
        #[rustfmt::skip]
        let visitor_palette= Table::create()
            .table(VisitorsPalettes::Table)
            .if_not_exists()
            .col(ColumnDef::new(VisitorsPalettes::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(VisitorsPalettes::VisitorId).integer().not_null())
            .col(ColumnDef::new(VisitorsPalettes::PaletteIdList).array(ColumnType::Integer).not_null())
            .col(ColumnDef::new(VisitorsPalettes::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(VisitorsPalettes::UpdatedAt).date_time())
            .foreign_key(foreign_key!(VisitorsPalettes::VisitorId to Visitors::Id Cascade))
            .to_owned();

        visitor_palette
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let visitor_palette = Self::up_visitor_palette();

        manager.create_table(visitor_palette).await?;

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
            VisitorsPalettes
        }

        Ok(())
    }
}

#[derive(Iden)]
enum VisitorsPalettes {
    Table,
    Id,
    VisitorId,
    PaletteIdList,
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
    fn test_visitor_palette_table() {
        let visitor_palette = Migration::up_visitor_palette();

        assert_eq!(
            visitor_palette.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "visitors_palettes" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""visitor_id" integer NOT NULL,"#,
                r#""palette_id_list" integer[] NOT NULL,"#,
                r#""created_at" timestamp without time zone NOT NULL,"#,
                r#""updated_at" timestamp without time zone,"#,
                r#"FOREIGN KEY ("visitor_id") REFERENCES "visitors" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#
            ].join(" ")
        )
    }
}
