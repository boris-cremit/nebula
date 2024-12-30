use std::sync::Arc;

use async_trait::async_trait;
use nebula_token::claim::NebulaClaim;
use sea_orm::DatabaseConnection;

use crate::{
    database::{Persistable, WorkspaceScopedTransaction},
    domain::secret::{self, AppliedPolicy, Path, SecretService},
};

pub(crate) struct PathData {
    pub path: String,
    pub applied_policies: Vec<AppliedPolicy>,
}

#[async_trait]
pub(crate) trait PathUseCase {
    async fn get_all(&self) -> Result<Vec<PathData>>;
    async fn register(&self, path: &str, policies: &[AppliedPolicy], claim: &NebulaClaim) -> Result<()>;
    async fn delete(&self, path: &str, claim: &NebulaClaim) -> Result<()>;
    async fn update(
        &self,
        path: &str,
        new_path: Option<&str>,
        new_policies: Option<&[AppliedPolicy]>,
        claim: &NebulaClaim,
    ) -> Result<()>;
    async fn get(&self, path: &str) -> Result<PathData>;
}

pub(crate) struct PathUseCaseImpl {
    workspace_name: String,
    database_connection: Arc<DatabaseConnection>,
    secret_service: Arc<dyn SecretService + Sync + Send>,
}

impl PathUseCaseImpl {
    pub fn new(
        workspace_name: String,
        database_connection: Arc<DatabaseConnection>,
        secret_service: Arc<dyn SecretService + Sync + Send>,
    ) -> Self {
        Self { workspace_name, database_connection, secret_service }
    }
}

#[async_trait]
impl PathUseCase for PathUseCaseImpl {
    async fn get_all(&self) -> Result<Vec<PathData>> {
        let transaction = self.database_connection.begin_with_workspace_scope(&self.workspace_name).await?;
        let paths = self.secret_service.get_paths(&transaction).await?;
        transaction.commit().await?;

        Ok(paths.into_iter().map(PathData::from).collect())
    }

    async fn register(&self, path: &str, policies: &[AppliedPolicy], claim: &NebulaClaim) -> Result<()> {
        let transaction = self.database_connection.begin_with_workspace_scope(&self.workspace_name).await?;
        self.secret_service.register_path(&transaction, path, policies, claim).await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn delete(&self, path: &str, claim: &NebulaClaim) -> Result<()> {
        let transaction = self.database_connection.begin_with_workspace_scope(&self.workspace_name).await?;
        let mut path = self
            .secret_service
            .get_path(&transaction, path)
            .await?
            .ok_or_else(|| Error::PathNotExists { entered_path: path.to_owned() })?;

        path.delete(&transaction, claim).await?;
        path.persist(&transaction).await?;

        transaction.commit().await?;
        Ok(())
    }

    async fn update(
        &self,
        path: &str,
        new_path: Option<&str>,
        new_policies: Option<&[AppliedPolicy]>,
        claim: &NebulaClaim,
    ) -> Result<()> {
        let transaction = self.database_connection.begin_with_workspace_scope(&self.workspace_name).await?;
        let mut path = self
            .secret_service
            .get_path(&transaction, path)
            .await?
            .ok_or_else(|| Error::PathNotExists { entered_path: path.to_owned() })?;

        if let Some(new_path) = new_path {
            path.update_path(&transaction, new_path, claim).await?;
        }
        if let Some(new_policies) = new_policies {
            path.update_policies(&transaction, new_policies, claim).await?;
        }

        path.persist(&transaction).await?;

        transaction.commit().await?;
        Ok(())
    }

    async fn get(&self, path: &str) -> Result<PathData> {
        let transaction = self.database_connection.begin_with_workspace_scope(&self.workspace_name).await?;
        let path = self
            .secret_service
            .get_path(&transaction, path)
            .await?
            .ok_or_else(|| Error::PathNotExists { entered_path: path.to_owned() })?;
        transaction.commit().await?;

        Ok(path.into())
    }
}

impl From<Path> for PathData {
    fn from(value: Path) -> Self {
        Self { path: value.path, applied_policies: value.applied_policies }
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Path({entered_path}) is in use")]
    PathIsInUse { entered_path: String },
    #[error("Path({entered_path}) is not registered")]
    PathNotExists { entered_path: String },
    #[error("Entered path({entered_path}) is already registered")]
    PathDuplicated { entered_path: String },
    #[error("Parent path for Path({entered_path}) is not registered")]
    ParentPathNotExists { entered_path: String },
    #[error("Invalid path({entered_path}) is entered")]
    InvalidPath { entered_path: String },
    #[error("Invalid path policy expression is entered")]
    InvalidPathPolicy,
    #[error("Access denied")]
    AccessDenied,
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl From<secret::Error> for Error {
    fn from(value: secret::Error) -> Self {
        match value {
            secret::Error::InvalidSecretIdentifier { .. } => Self::Anyhow(value.into()),
            secret::Error::SecretNotExists => Self::Anyhow(value.into()),
            secret::Error::Anyhow(e) => Self::Anyhow(e),
            secret::Error::IdentifierConflicted { .. } => Self::Anyhow(value.into()),
            secret::Error::InvalidPath { entered_path } => Self::InvalidPath { entered_path },
            secret::Error::ParentPathNotExists { entered_path } => Self::ParentPathNotExists { entered_path },
            secret::Error::PathDuplicated { entered_path } => Self::PathDuplicated { entered_path },
            secret::Error::PathIsInUse { entered_path } => Self::PathIsInUse { entered_path },
            secret::Error::InvalidPathPolicy => Self::InvalidPathPolicy,
            secret::Error::AccessDenied => Self::AccessDenied,
            secret::Error::InvalidSecretPolicy => Self::Anyhow(value.into()),
        }
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Error::Anyhow(value.into())
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use chrono::Utc;
    use nebula_token::claim::{NebulaClaim, Role};
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use ulid::Ulid;

    use crate::{
        database::{applied_path_policy, path, secret_metadata, secret_value, UlidId},
        domain::secret::{MockSecretService, Path},
    };

    use super::{Error, PathUseCase, PathUseCaseImpl};

    #[tokio::test]
    async fn when_getting_paths_is_successful_then_policy_usecase_returns_paths_ok() {
        let path = "/frontend";

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_paths()
            .withf(|_| true)
            .times(1)
            .returning(move |_| Ok(vec![Path::new(path.to_owned(), vec![])]));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        let result = path_usecase.get_all().await.expect("creating workspace should be successful");

        assert_eq!(result[0].path, path);
    }

    #[tokio::test]
    async fn when_getting_secret_is_failed_with_anyhow_then_secret_usecase_returns_anyhow_err() {
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_paths()
            .withf(|_| true)
            .times(1)
            .returning(move |_| Err(crate::domain::secret::Error::Anyhow(anyhow::anyhow!("some error"))));
        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        let result = path_usecase.get_all().await;

        assert!(matches!(result, Err(Error::Anyhow(_))));
        assert_eq!(result.err().unwrap().to_string(), "some error");
    }

    #[tokio::test]
    async fn when_registering_path_is_successful_then_secret_usecase_returns_unit_ok() {
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service.expect_register_path().times(1).returning(move |_, _, _, _| Ok(()));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        path_usecase.register(path, &[], &claim).await.expect("registering path should be successful");
    }

    #[tokio::test]
    async fn when_deleting_existing_path_then_path_usecase_returns_unit_ok() {
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";
        let now = Utc::now();

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test/path".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }])
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_query_results([
                [maplit::btreemap! {
                    "num_items" => sea_orm::Value::BigInt(Some(0))
                }],
                [maplit::btreemap! {
                    "num_items" => sea_orm::Value::BigInt(Some(0))
                }],
            ])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }])
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: path.to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_path()
            .times(1)
            .returning(move |_, _| Ok(Some(Path::new(path.to_owned(), vec![]))));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        path_usecase.delete(path, &claim).await.expect("registering path should be successful");
    }

    #[tokio::test]
    async fn when_deleting_existing_path_having_child_path_then_path_usecase_returns_path_is_in_use_err() {
        let now = Utc::now();
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test/path".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }])
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_query_results([
                [maplit::btreemap! {
                    "num_items" => sea_orm::Value::BigInt(Some(1))
                }],
                [maplit::btreemap! {
                    "num_items" => sea_orm::Value::BigInt(Some(0))
                }],
            ])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_path()
            .times(1)
            .returning(move |_, _| Ok(Some(Path::new(path.to_owned(), vec![]))));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        let result = path_usecase.delete(path, &claim).await;

        assert!(matches!(result, Err(Error::PathIsInUse { .. })))
    }

    #[tokio::test]
    async fn when_deleting_existing_path_having_child_secret_then_path_usecase_returns_path_is_in_use_err() {
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";
        let now = Utc::now();

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test/path".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }])
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_query_results([
                [maplit::btreemap! {
                    "num_items" => sea_orm::Value::BigInt(Some(0))
                }],
                [maplit::btreemap! {
                    "num_items" => sea_orm::Value::BigInt(Some(1))
                }],
            ])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_path()
            .times(1)
            .returning(move |_, _| Ok(Some(Path::new(path.to_owned(), vec![]))));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        let result = path_usecase.delete(path, &claim).await;

        assert!(matches!(result, Err(Error::PathIsInUse { .. })))
    }

    #[tokio::test]
    async fn when_deleting_not_existing_path_then_path_usecase_returns_path_not_exists_err() {
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service.expect_get_path().times(1).returning(move |_, _| Ok(None));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        let result = path_usecase.delete(path, &claim).await;

        assert!(matches!(result, Err(Error::PathNotExists { .. })));
    }

    #[tokio::test]
    async fn when_updating_existing_path_then_path_usecase_returns_unit_ok() {
        let now = Utc::now();
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }])
            .append_query_results([[maplit::btreemap! {
                "num_items" => sea_orm::Value::BigInt(Some(0))
            }]])
            .append_query_results([Vec::<path::Model>::new()])
            .append_query_results([Vec::<secret_metadata::Model>::new()])
            .append_query_results([Vec::<secret_value::Model>::new()])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_path()
            .times(1)
            .returning(move |_, _| Ok(Some(Path::new(path.to_owned(), vec![]))));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        path_usecase
            .update(path, Some("/new/test/path"), None, &claim)
            .await
            .expect("registering path should be successful");
    }

    #[tokio::test]
    async fn when_updating_existing_path_to_existing_path_then_path_usecase_returns_path_duplicated_err() {
        let now = Utc::now();
        let claim = NebulaClaim {
            gid: "test@cremit.io".to_owned(),
            workspace_name: "cremit".to_owned(),
            attributes: HashMap::new(),
            role: Role::Member,
        };

        let path = "/test/path";

        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/test".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_query_results([[path::Model {
                id: UlidId::new(Ulid::new()),
                path: "/".to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_query_results([Vec::<applied_path_policy::Model>::new()])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }])
            .append_query_results([[maplit::btreemap! {
                "num_items" => sea_orm::Value::BigInt(Some(1))
            }]])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 1 }]);

        let mock_connection = Arc::new(mock_database.into_connection());

        let mut mock_secret_service = MockSecretService::new();
        mock_secret_service
            .expect_get_path()
            .times(1)
            .returning(move |_, _| Ok(Some(Path::new(path.to_owned(), vec![]))));

        let path_usecase =
            PathUseCaseImpl::new("testworkspace".to_owned(), mock_connection, Arc::new(mock_secret_service));

        let result = path_usecase.update(path, Some("/new/test/path"), None, &claim).await;

        assert!(matches!(result, Err(Error::PathDuplicated { .. })))
    }
}
