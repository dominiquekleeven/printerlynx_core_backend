use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agent::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Agent::Uuid).string().not_null().primary_key())
                    .col(ColumnDef::new(Agent::UserUuid).string().not_null())
                    .col(ColumnDef::new(Agent::Name).string().not_null())
                    .col(ColumnDef::new(Agent::Description).string().not_null())
                    .col(ColumnDef::new(Agent::Token).string().not_null())
                    .col(ColumnDef::new(Agent::CreatedAt).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Agent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Agent {
    Table,
    Uuid,
    UserUuid,
    Name,
    Description,
    Token,
    CreatedAt,
}
