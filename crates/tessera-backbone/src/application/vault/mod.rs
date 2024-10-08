use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::domain::vault::{Error as VaultServiceError, VaultService};

use self::command::CreatingVaultCommand;

pub mod command;

#[async_trait]
pub(crate) trait VaultUseCase {
    async fn create(&self, cmd: CreatingVaultCommand) -> Result<()>;
}

#[derive(Default)]
pub(crate) struct VaultUseCaseImpl<V: VaultService + Sync + Send> {
    database_connection: Arc<DatabaseConnection>,
    vault_service: Arc<V>,
}

impl<V: VaultService + Sync + Send> VaultUseCaseImpl<V> {
    pub fn new(database_connection: Arc<DatabaseConnection>, vault_service: Arc<V>) -> Self {
        Self { database_connection, vault_service }
    }
}

#[async_trait]
impl<V: VaultService + Sync + Send> VaultUseCase for VaultUseCaseImpl<V> {
    async fn create(&self, cmd: CreatingVaultCommand) -> Result<()> {
        let transaction = self.database_connection.begin().await.map_err(anyhow::Error::from)?;

        self.vault_service.create(&transaction, &cmd.name).await?;

        transaction.commit().await.map_err(anyhow::Error::from)?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl From<VaultServiceError> for Error {
    fn from(value: VaultServiceError) -> Self {
        unreachable!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use sea_orm::{DatabaseBackend, MockDatabase};

    use crate::{
        application::vault::{command::CreatingVaultCommand, VaultUseCase, VaultUseCaseImpl},
        domain::vault::MockVaultService,
    };

    #[tokio::test]
    async fn when_creating_vault_use_case_should_delegate_to_service() {
        const VAULT_NAME: &'static str = "test_vault";
        let mock_database = Arc::new(MockDatabase::new(DatabaseBackend::Postgres).into_connection());
        let mut vault_service_mock = MockVaultService::new();

        vault_service_mock.expect_create().withf(|_, name| name == VAULT_NAME).times(1).returning(|_, _| Ok(()));

        let vault_use_case = VaultUseCaseImpl::new(mock_database, Arc::new(vault_service_mock));
        vault_use_case
            .create(CreatingVaultCommand { name: VAULT_NAME.to_owned() })
            .await
            .expect("creating vault should be successful");
    }
}
