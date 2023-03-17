pub use sea_orm_migration::prelude::*;

mod m20230307_212009_create_users;
mod m20230307_212813_create_playlists;
mod m20230307_215816_create_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230307_212009_create_users::Migration),
            Box::new(m20230307_212813_create_playlists::Migration),
            Box::new(m20230307_215816_create_sessions::Migration),
        ]
    }
}
