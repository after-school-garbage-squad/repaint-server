pub use sea_orm_migration::prelude::*;

mod m20230712_175819_create_event_table;
mod m20230712_180346_create_event_image_table;
mod m20230712_213646_create_event_spot_table;
mod m20230712_214136_create_event_beacon_table;
mod m20230712_215023_create_visitor_table;
mod m20230712_215554_create_visitor_image_table;
mod m20230712_220812_create_visitor_palette_table;
mod m20230712_222322_create_admin_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230712_175819_create_event_table::Migration),
            Box::new(m20230712_180346_create_event_image_table::Migration),
            Box::new(m20230712_213646_create_event_spot_table::Migration),
            Box::new(m20230712_214136_create_event_beacon_table::Migration),
            Box::new(m20230712_215023_create_visitor_table::Migration),
            Box::new(m20230712_215554_create_visitor_image_table::Migration),
            Box::new(m20230712_220812_create_visitor_palette_table::Migration),
            Box::new(m20230712_222322_create_admin_table::Migration),
        ]
    }
}
