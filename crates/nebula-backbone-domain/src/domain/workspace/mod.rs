use async_trait::async_trait;

mod workspace_service;

use sea_orm::{DatabaseTransaction, EntityTrait};
use tracing::info;
use ulid::Ulid;
#[cfg(test)]
pub use workspace_service::MockWorkspaceService;
pub use workspace_service::{WorkspaceService, WorkspaceServiceImpl};

use crate::database::{Persistable, UlidId};

#[derive(Debug, PartialEq)]
pub struct Workspace {
    id: Ulid,
    pub name: String,
    deleted: bool,
}

impl Workspace {
    pub fn new(id: Ulid, name: String) -> Self {
        Self { id, name, deleted: false }
    }

    pub fn delete(&mut self) {
        self.deleted = true
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("workspace name already exists")]
    WorkspaceNameConflicted,

    #[error("invalid workspace name")]
    InvalidWorkspaceName,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl Persistable for Workspace {
    type Error = crate::domain::workspace::Error;

    async fn persist(self, transaction: &DatabaseTransaction) -> crate::domain::workspace::Result<()> {
        if self.deleted {
            use crate::database::workspace::Entity;

            Entity::delete_by_id(UlidId::from(self.id)).exec(transaction).await?;

            let workspace_name = self.name;
            info!("workspace(name: {workspace_name}) is deleted.");
            return Ok(());
        };

        Ok(())
    }
}
