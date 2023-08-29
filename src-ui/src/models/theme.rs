use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "themes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Settings,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Settings => Entity::belongs_to(super::settings::Entity)
                .linking(super::settings::Column::ThemeSelected) // <-- Note: using `linking` here
                .into(),
        }
    }
}

impl Related<super::settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Settings.def()
    }
}

#[async_trait::async_trait] // <-- Add this
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(&self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

impl EntityTrait for Entity {
    type Model = Model;
    type Relation = Relation;
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "themes"
    }
}
