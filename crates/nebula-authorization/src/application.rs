use std::sync::Arc;

use crate::{
    config::{ApplicationConfig, StaticWorkspaceConfig, UpstreamIdpConfig, WorkspaceConfig},
    database::{self, connect_to_database, AuthMethod},
    domain::{
        connector::saml::{SAMLConnector, SAMLConnertorConfig},
        machine_identity::MachineIdentityService,
        token::TokenService,
        workspace::WorkspaceService,
    },
};

use nebula_token::jwk::jwk_set::{JwkSet, JWK_SET_DEFAULT_KEY_ID};
use sea_orm::{DatabaseConnection, TransactionTrait};

pub struct Application {
    pub database_connection: Arc<DatabaseConnection>,
    pub connector: Arc<SAMLConnector>,
    pub token_service: Arc<TokenService>,
    pub machine_identity_service: Arc<MachineIdentityService>,
    pub workspace_service: Arc<WorkspaceService>,
}

impl Application {
    pub async fn init(config: &ApplicationConfig) -> anyhow::Result<Self> {
        let database_connection = init_database_connection(config).await?;

        match config.workspace {
            WorkspaceConfig::Static(StaticWorkspaceConfig { ref name }) => {
                database::migrate_workspace(
                    name,
                    &config.database.host,
                    config.database.port,
                    &config.database.database_name,
                    &create_database_auth_method(config),
                )
                .await?;
            }
            WorkspaceConfig::Claim(_) => {
                database::migrate_all_workspaces(
                    &database_connection.begin().await?,
                    &config.database.host,
                    config.database.port,
                    &config.database.database_name,
                    &create_database_auth_method(config),
                )
                .await?;
            }
        }

        let saml_config = match config.upstream_idp {
            UpstreamIdpConfig::Saml(ref saml) => {
                let redirect_uri = if let Some(ref path_prefix) = config.path_prefix {
                    let path_prefix = format!("{}/", path_prefix.trim_matches('/'));
                    config.base_url.join(&path_prefix)?.join("callback/saml")?
                } else {
                    config.base_url.join("callback/saml")?
                };

                SAMLConnertorConfig::builder()
                    .redirect_uri(redirect_uri)
                    .maybe_sso_url(saml.sso_url.as_ref())
                    .maybe_idp_issuer(saml.idp_issuer.as_ref())
                    .entity_id(&saml.entity_id)
                    .ca(openssl::x509::X509::from_pem(saml.ca.as_bytes())?)
                    .attributes_config(saml.attributes.clone())
                    .workspace_config(config.workspace.clone())
                    .admin_role_config(saml.admin_role.clone())
                    .build()
            }
        };

        let saml_connector = Arc::new(SAMLConnector::new(saml_config)?);

        let (jwks, kid) = match (&config.token.jwks, &config.token.jwk_kid) {
            (Some(jwks), Some(kid)) => (jwks.clone(), kid.clone()),
            (Some(jwks), None) => (jwks.clone(), JWK_SET_DEFAULT_KEY_ID.to_string()),
            _ => (JwkSet::default(), JWK_SET_DEFAULT_KEY_ID.to_string()),
        };

        Ok(Self {
            database_connection: database_connection.clone(),
            connector: saml_connector,
            token_service: Arc::new(TokenService::new(config.base_url.clone(), config.token.lifetime, jwks, kid)),
            machine_identity_service: Arc::new(MachineIdentityService {}),
            workspace_service: Arc::new(WorkspaceService::new(
                database_connection.clone(),
                config.database.host.to_owned(),
                config.database.port,
                config.database.database_name.to_owned(),
                create_database_auth_method(config),
            )),
        })
    }
}

async fn init_database_connection(config: &ApplicationConfig) -> anyhow::Result<Arc<DatabaseConnection>> {
    let database_host = &config.database.host;
    let database_port = config.database.port;
    let database_name = &config.database.database_name;
    let auth_method = create_database_auth_method(config);

    connect_to_database(database_host, database_port, database_name, &auth_method).await
}

fn create_database_auth_method(config: &ApplicationConfig) -> AuthMethod {
    match &config.database.auth {
        crate::config::DatabaseAuthConfig::Credential { username, password } => {
            AuthMethod::Credential { username: username.to_owned(), password: password.to_owned() }
        }
        crate::config::DatabaseAuthConfig::RdsIamAuth { username } => AuthMethod::RdsIamAuth {
            host: config.database.host.to_owned(),
            port: config.database.port,
            username: username.to_owned(),
        },
    }
}
