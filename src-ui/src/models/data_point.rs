use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "data_points")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project: i32,
    pub data: f64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "data_points"
    }
}

impl EntityTrait for Entity {
    type Model = Model;

    type Relation = ();
}
