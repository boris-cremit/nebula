use chrono::{DateTime, Utc};
use sea_orm::prelude::*;

use super::UlidId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "applied_policy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UlidId,
    pub secret_metadata_id: UlidId,
    pub policy_id: UlidId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::secret_metadata::Entity",
        from = "Column::SecretMetadataId",
        to = "super::secret_metadata::Column::Id"
    )]
    SecretMetadata,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::secret_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SecretMetadata.def()
    }
}
