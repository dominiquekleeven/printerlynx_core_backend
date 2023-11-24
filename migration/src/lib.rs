pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_account;
mod m20230916_170023_create_table_printfile;
mod m20231007_204738_alter_printfile_add_size;
mod m20231124_134301_create_table_agent;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_account::Migration),
            Box::new(m20230916_170023_create_table_printfile::Migration),
            Box::new(m20231007_204738_alter_printfile_add_size::Migration),
            Box::new(m20231124_134301_create_table_agent::Migration),
        ]
    }
}
