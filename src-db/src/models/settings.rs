use crate::models::settings;
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
            Self::Theme => Entity::has_many(super::theme::Entity).into(),
        }
    }
}

impl Related<super::theme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Theme.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for settings::ActiveModel {
    async fn before_save<C: ConnectionTrait>(self, _db: &C, _insert: bool) -> Result<Self, DbErr> {
        // Your logic here
        Ok(self)
    }
}
