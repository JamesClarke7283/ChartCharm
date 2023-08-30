use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelBehavior; // Keep this if you need custom behavior

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
                .from(Column::Id)
                .to(super::settings::Column::ThemeSelected)
                .into(),
        }
    }
}

impl Related<super::settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Settings.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C: ConnectionTrait>(
        self, // <- Changed from &mut self
        _db: &C,
        _insert: bool,
    ) -> Result<Self, DbErr> {
        // <- Return type also updated
        // Your logic here
        Ok(self) // <- Returns ownership to caller
    }
}
