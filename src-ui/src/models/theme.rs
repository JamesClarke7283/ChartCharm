use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "themes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::settings::Entity",
        from = "Column::Id",
        to = "super::settings::Column::ThemeSelected"
    )]
    Settings,
}

pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "themes"
    }
}

impl EntityTrait for Entity {
    type Model = Model;

    type Relation = Relation;
}
