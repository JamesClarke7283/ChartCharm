use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub theme_selected: i32,
}

pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "settings"
    }
}

impl EntityTrait for Entity {
    type Model = Model;

    type Relation = ();
}
