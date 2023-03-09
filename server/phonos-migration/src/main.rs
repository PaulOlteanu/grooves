use sea_orm_migration::prelude::*;

// mod migration;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
