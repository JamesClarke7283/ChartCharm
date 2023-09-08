use chrono::Utc;
use sea_orm::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "charts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u16,
    pub name: String,
    pub description: String,
    pub project: u16,
    pub kind: u8,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::Project",
        to = "super::projects::Column::Id"
    )]
    Project,
    #[sea_orm(
        belongs_to = "super::chart_kind::Entity",
        from = "Column::Kind",
        to = "super::chart_kind::Column::Id"
    )]
    Kind,
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
