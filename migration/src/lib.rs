pub use sea_orm_migration::prelude::*;

mod m20220810_130049_create_txo;
mod m20220810_131915_create_asset;
mod m20220810_132240_create_batch_transfer;
mod m20220810_132250_create_asset_transfer;
mod m20220810_132253_create_coloring;
mod m20220810_132256_create_transfer;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220810_130049_create_txo::Migration),
            Box::new(m20220810_131915_create_asset::Migration),
            Box::new(m20220810_132240_create_batch_transfer::Migration),
            Box::new(m20220810_132250_create_asset_transfer::Migration),
            Box::new(m20220810_132253_create_coloring::Migration),
            Box::new(m20220810_132256_create_transfer::Migration),
        ]
    }
}
