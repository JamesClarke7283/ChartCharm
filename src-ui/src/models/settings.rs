use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Settings {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub theme_selected: i32,
}
