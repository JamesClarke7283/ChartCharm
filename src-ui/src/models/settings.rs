use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub theme_selected: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Theme,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Theme => Entity::belongs_to(super::theme::Entity)
                .using(super::theme::Column::Id)
                .into(),
        }
    }
}

impl Related<super::theme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Theme.def()
    }
}
