use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // related to data_points, project
    #[sea_orm(
        has_many = "super::data_points::Entity",
        from = "Column::Id",
        to = "super::data_points::Column::Project"
    )]
    DataPoints,
}

impl ActiveModelBehavior for ActiveModel {}
