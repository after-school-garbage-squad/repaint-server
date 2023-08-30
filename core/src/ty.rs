pub mod json;
pub mod string;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use email_address::EmailAddress;
    use pretty_assertions::*;
    use repaint_server_model::event::Contact;
    use repaint_server_model::id::Id;
    use sea_orm::sea_query::{self, ColumnDef, Iden, Table};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{
        ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DeriveEntityModel,
        DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
    };

    use crate::ty::json::AsJson;
    use crate::ty::string::AsString;
    use crate::TestingSeaOrm;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "json_test")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: AsString<Id<Model>>,
        pub json: AsJson<Contact>,
    }

    impl ActiveModelBehavior for ActiveModel {}

    #[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[derive(Iden)]
    enum JsonTest {
        Table,
        Id,
        Json,
    }

    #[test_log::test(tokio::test)]
    async fn test() {
        let orm = TestingSeaOrm::new().await;
        let con = orm.orm().con();

        #[rustfmt::skip]
        con.execute(
            con.get_database_backend().build(
                Table::create()
                    .table(JsonTest::Table)
                    .col(ColumnDef::new(JsonTest::Id).string().primary_key().not_null())
                    .col(ColumnDef::new(JsonTest::Json).json().not_null())
            ),
        )
        .await
        .unwrap();

        let id = Id::new();
        let contact = Contact {
            name: "test".into(),
            email: EmailAddress::from_str("test@example.com").unwrap(),
            phone: "0120-10-7929".into(),
        };

        ActiveModel {
            id: Set(AsString(id)),
            json: Set(AsJson(contact.clone())),
        }
        .insert(con)
        .await
        .unwrap();

        self::assert_eq!(
            Entity::find_by_id(AsString(id))
                .one(con)
                .await
                .unwrap()
                .unwrap(),
            Model {
                id: AsString(id),
                json: AsJson(contact),
            }
        );
    }
}
