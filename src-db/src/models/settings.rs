use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub theme_selected: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::theme::Entity",
        from = "Column::ThemeSelected",
        to = "super::theme::Column::Id"
    )]
    Theme,
}

impl ActiveModelBehavior for ActiveModel {}
