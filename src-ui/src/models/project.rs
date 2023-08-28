use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::data_point::Entity",
        from = "Column::Id",
        to = "super::data_point::Column::Project"
    )]
    DataPoint,
}

pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "projects"
    }
}

impl EntityTrait for Entity {
    type Model = Model;

    type Relation = Relation;
}
