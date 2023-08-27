//! The [`Config`] struct represents a full server configuration.

use crate::db::DbConfig;
use crate::env::Environment;
use crate::server::HttpServerConfig;
use anyhow::Result;
use log::{debug, log, Level};
use std::env;
use std::fmt::Debug;

/// `Config` is responsible of the configuration of a "full" server,
/// reading the settings from environment variables: app environment,
/// HTTP server settings and database settings.
///
/// It does not setup anything related with the data layer
/// other than the DB string connection, see the `app_state`
/// module for that.
/// The web layer configuration is configured by `app_server`.
#[derive(Debug, Clone)]
pub struct Config {
    /// The environment name chosen to run the app, normally
    /// set through the environment variable `APP_ENV`.
    /// See [`Environment::init()`].
    pub env: Environment,
    /// All the config needed to launch an HTTP server.
    pub server: HttpServerConfig,
    /// All the config needed to launch an HTTP server.
    pub db: DbConfig,
}

impl Config {
    /// Initialize all the configurations, setting each value with its corresponding
    /// environment variable, e.g. the `env` attribute with the `APP_ENV` environment variable.
    ///
    /// The port number is get from the `PORT` env variable, otherwise
    /// defaulted to `default_port`.
    ///
    /// # Examples
    /// ```
    /// use std::env;
    /// use server_env_config::Config;
    /// use server_env_config::env::Environment;
    ///
    /// // Settings should be actually set by the OS environment
    /// env::set_var("APP_ENV", "production");
    /// env::set_var("APP_URI", "api/v1");
    /// env::set_var("PORT", "8080");
    /// env::set_var("DATABASE_URL", "postgresql://user:pass@localhost/db");
    ///
    /// let result = Config::init(999);
    /// assert!(result.is_ok());
    /// let config = result.unwrap();
    /// assert_eq!(config.env, Environment::Production);
    /// assert_eq!(config.server.port, 8080);
    /// assert_eq!(config.server.url, "http://127.0.0.1:8080/api/v1/");    // calculated field
    /// assert_eq!(config.db.database_url, "postgresql://user:pass@localhost/db");
    /// // Some settings have default values if env variables are not set
    /// assert_eq!(config.db.min_connections, 1);
    /// assert_eq!(config.db.max_connections, 10);
    /// ```
    pub fn init(default_port: u16) -> Result<Config> {
        Self::init_for(default_port, None)
    }

    /// Initialize config with the environment passed, if `None`, env
    /// will be set with the `APP_ENV` environment variable.
    ///
    /// The port number is get from the `PORT` env variable, otherwise
    /// defaulted to `default_port`.
    ///
    /// See [`Config::init()`].
    pub fn init_for(default_port: u16, environment: Option<Environment>) -> Result<Config> {
        debug!("⚙️  Configuring Backset ...");
        let env = match environment {
            Some(e) => e,
            None => Environment::init()?,
        };
        let log_level = match env {
            Environment::Test => Level::Debug,
            _ => Level::Info,
        };
        log!(log_level, "⚙️  Environment set to {env}");
        let db = DbConfig::init_for(&env)?;
        let server = HttpServerConfig::init_for("127.0.0.1", default_port)?;
        Ok(Config { env, server, db })
    }
}

impl ToString for Config {
    /// This `to_string()` implementation prints out all the config
    /// values in `.env` format, using as key the environment variable
    /// used to set-up the config, even if the configuration was
    /// set in another way, e.g. using a default value.
    fn to_string(&self) -> String {
        format!(
r#"# The following items are the environment variables and its values from
# the OS, from an .env file, or the default value used by **Backset**.
#
# APP_URL --> {}
#
APP_ENV={}
APP_URI="{}"
HOST={}
PORT={}
DATABASE_URL="{}"
MIN_CONNECTIONS={}
MAX_CONNECTIONS={}
ACQUIRE_TIMEOUT_MS={}
IDLE_TIMEOUT_SEC={}
TEST_BEFORE_ACQUIRE={}
RUST_LOG="{}""#,
            self.server.url,
            self.env,
            self.server.uri,
            self.server.addr,
            self.server.port,
            self.db.database_url,
            self.db.min_connections,
            self.db.max_connections,
            self.db.acquire_timeout.as_millis(),
            self.db.idle_timeout.as_secs(),
            self.db.test_before_acquire,
            env::var("RUST_LOG").unwrap_or("".to_string())
        )
    }
}
