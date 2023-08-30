use crate::models::data_point;
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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Project,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Project => Entity::belongs_to(super::project::Entity).into(),
        }
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for data_point::ActiveModel {
    async fn before_save<C: ConnectionTrait>(self, _db: &C, _insert: bool) -> Result<Self, DbErr> {
        // Your logic here
        Ok(self)
    }
}
