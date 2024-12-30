use chrono::{DateTime, Utc};
use sea_orm::prelude::*;

use super::UlidId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "secret_metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UlidId,
    pub key: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::applied_policy::Entity")]
    AppliedPolicy,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::applied_policy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AppliedPolicy.def()
    }
}
