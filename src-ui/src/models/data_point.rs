use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "data_points")]
pub struct DataPoint {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project: i32,
    pub data: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
