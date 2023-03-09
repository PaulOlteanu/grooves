use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Playlist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Playlist::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Playlist::Name).string().not_null())
                    .col(ColumnDef::new(Playlist::OwnerId).integer().not_null())
                    .col(ColumnDef::new(Playlist::Elements).json_binary())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(Playlist::Table, Playlist::OwnerId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Playlist::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Playlist {
    Table,
    Id,
    Name,
    OwnerId,
    Elements,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
