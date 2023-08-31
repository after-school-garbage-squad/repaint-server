use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(repaint_server_migration::Migrator).await;
}
