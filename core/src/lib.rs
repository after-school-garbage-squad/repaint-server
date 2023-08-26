#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![feature(min_specialization)]

use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use teloc::dev::DependencyClone;

pub mod entity;
pub mod impls;
pub mod ty;

#[derive(Debug, Clone)]
pub struct SeaOrm {
    con: Arc<DatabaseConnection>,
}

impl SeaOrm {
    pub async fn new(opt: impl Into<ConnectOptions>) -> Result<Self, DbErr> {
        let con = Arc::new(Database::connect(opt).await?);
        Ok(SeaOrm { con })
    }

    pub fn con(&self) -> &DatabaseConnection {
        &self.con
    }
}

impl DependencyClone for SeaOrm {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("seaorm がエラーを返しました: {0}")]
    SeaOrm(#[from] DbErr),
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use repaint_server_migration::{Migrator, MigratorTrait};
        use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
        use sqlx::postgres::PgConnection;
        use sqlx::Connection;
        use tokio::runtime::Handle;

        // make isolated testing environment

        pub(crate) struct TestingSeaOrm {
            orm: SeaOrm,
            db_name: String,
        }

        impl TestingSeaOrm {
            pub(crate) async fn new() -> Self {
                const TESTING_POSTGRES_URL_BASE: &str = "postgresql://user:pass@localhost";
                const TESTING_POSTGRES_URL: &str = "postgresql://user:pass@localhost/local-db";

                let db_name = ulid::Ulid::new().to_string();

                {
                    let mut con = PgConnection::connect(TESTING_POSTGRES_URL).await.unwrap();
                    sqlx::query(&format!("CREATE DATABASE \"{}\"", db_name)).execute(&mut con).await.unwrap();
                }

                let url = format!("{}/{}", TESTING_POSTGRES_URL_BASE, &db_name);
                let options = ConnectOptions::new(url);
                let orm = SeaOrm::new(options).await.unwrap();

                Migrator::up(orm.con(), None).await.unwrap();

                Self { orm, db_name }
            }

            pub(crate) fn orm(&self) -> &SeaOrm {
                &self.orm
            }
        }

        impl Drop for TestingSeaOrm {
            fn drop(&mut self) {
                let db_name = self.db_name.clone();
                let orm = self.orm.clone();

                Handle::current().spawn(async move {
                    let stmt = Statement::from_string(
                        DatabaseBackend::Postgres,
                        format!("DROP DATABASE {}", db_name),
                    );

                    orm.con().execute(stmt).await.unwrap();
                });
            }
        }
    }
}
