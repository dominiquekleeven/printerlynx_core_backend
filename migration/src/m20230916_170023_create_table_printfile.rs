use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PrintFile::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PrintFile::Uuid).string().not_null().primary_key(), )
                    .col(ColumnDef::new(PrintFile::UserUuid).string().not_null())
                    .col(ColumnDef::new(PrintFile::Name).string().not_null())
                    .col(ColumnDef::new(PrintFile::Path).string().not_null())
                    .col(ColumnDef::new(PrintFile::Checksum).string().not_null())
                    .col(ColumnDef::new(PrintFile::FileType).string().not_null(), )
                    .col(ColumnDef::new(PrintFile::FileStorageType).string().not_null(), )
                    .col(ColumnDef::new(PrintFile::CreatedAt).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PrintFile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PrintFile {
    Table,
    Uuid,
    UserUuid,
    Name,
    Path,
    Checksum,
    FileType,
    FileStorageType,
    CreatedAt,

}
