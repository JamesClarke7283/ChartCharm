use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "chart_kind")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u8,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::charts::Entity",
        from = "Column::Id",
        to = "super::charts::Column::Kind"
    )]
    Charts,
}

impl ActiveModelBehavior for ActiveModel {}
