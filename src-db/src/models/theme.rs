use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "theme")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // related to settings, theme_selected
    #[sea_orm(
        belongs_to = "super::settings::Entity",
        from = "Column::Id",
        to = "super::settings::Column::ThemeSelected"
    )]
    Settings,
}

impl ActiveModelBehavior for ActiveModel {}
