//! The [`DbConfig`] struct represents settings used to establish a connection with a database.

use crate::env::Environment;
use crate::{env_bool, env_parsable};
use anyhow::{Context, Result};
use std::env;
use std::time::Duration;

/// Settings used to establish a connection with a database, regardless of the engine.
/// All the values can be initialized with [`DbConfig::init_for()`] method, that uses
/// environment variables to set up all of them, otherwise all have default values,
/// except the string connection.
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database URL, initialized with the `DATABASE_URL` env
    pub database_url: String,
    /// Min connections created at start-up, value set with `MIN_CONNECTIONS` env,
    /// default 1
    pub min_connections: u32,
    /// Max connections allowed, value set with `MAX_CONNECTIONS` env,
    /// default 10
    pub max_connections: u32,
    /// Time allowed to acquire a connection, value set with `ACQUIRE_TIMEOUT_MS` env,
    /// default 750 milliseconds
    pub acquire_timeout: Duration,
    /// Max time a connection can be idle, value set with `IDLE_TIMEOUT_SEC` env,
    /// default 300 sec (5 min).
    /// Any connection that remains in the idle queue longer than this will be closed.
    pub idle_timeout: Duration,
    /// Whether to test before test the connection at start-up or not,
    /// value set with `TEST_BEFORE_ACQUIRE` env, default to false
    pub test_before_acquire: bool,
}

impl DbConfig {
    /// Init the object with `env` passed, and the rest of the
    /// attributes reading its corresponding environment variable,
    /// otherwise use a default value.
    ///
    /// The database string is saved in `self.database_url` with the value found at
    /// the `DATABASE_URL` environment value, that it's the only one required (there
    /// is no default value). If `env` passed is [`Environment::Test`] the prefix
    /// `_test` is added to the string connection, to avoid using by mistake prod/local
    /// databases, unless the string already ends with the prefix, or the string has
    /// connection arguments (the `?` symbol in the string).
    ///
    /// # Examples
    /// ```
    /// use std::env;
    /// use server_env_config::db::DbConfig;
    /// use server_env_config::env::Environment;
    ///
    /// // Configurations should be actually set by the OS environment
    /// env::set_var("DATABASE_URL", "postgresql://user:pass@localhost/db");
    /// env::set_var("MAX_CONNECTIONS", "50");
    /// env::set_var("IDLE_TIMEOUT_SEC", "60");
    ///
    /// let db = DbConfig::init_for(&Environment::Local).unwrap();
    ///
    /// assert_eq!(db.database_url, "postgresql://user:pass@localhost/db");
    /// assert_eq!(db.max_connections, 50);
    /// // All settings except DATABASE_URL have default values if env variables are not set
    /// assert_eq!(db.min_connections, 1);
    /// assert!(!db.test_before_acquire);
    ///
    /// env::remove_var("DATABASE_URL"); // if not set, DbConfig cannot be initialized
    /// let db = DbConfig::init_for(&Environment::Local);
    /// assert!(db.is_err());
    /// ```
    pub fn init_for(env: &Environment) -> Result<Self> {
        let url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let database_url = if *env == Environment::Test && !url.ends_with("_test") && !url.contains('?') {
            format!("{url}_test")
        } else {
            url
        };
        let min_connections = env_parsable::<u32>("MIN_CONNECTIONS", 1)?;
        let max_connections = env_parsable::<u32>("MAX_CONNECTIONS", 10)?;
        let acquire_timeout = Duration::from_millis(env_parsable::<u64>("ACQUIRE_TIMEOUT_MS", 750)?);
        let idle_timeout = Duration::from_secs(env_parsable::<u64>("IDLE_TIMEOUT_SEC", 300)?);
        let test_before_acquire = env_bool("TEST_BEFORE_ACQUIRE", false)?;
        Ok(DbConfig {
            database_url,
            min_connections,
            max_connections,
            acquire_timeout,
            idle_timeout,
            test_before_acquire,
        })
    }
}

impl ToString for DbConfig {
    /// This `to_string()` implementation prints out all the config
    /// values in `.env` format, using as key the environment variable
    /// used to set-up the config, even if the configuration was
    /// set in another way, e.g. using a default value.
    fn to_string(&self) -> String {
        format!(
r#"DATABASE_URL="{}"
MIN_CONNECTIONS={}
MAX_CONNECTIONS={}
ACQUIRE_TIMEOUT_MS={}
IDLE_TIMEOUT_SEC={}
TEST_BEFORE_ACQUIRE={}"#,
            self.database_url,
            self.min_connections,
            self.max_connections,
            self.acquire_timeout.as_millis(),
            self.idle_timeout.as_secs(),
            self.test_before_acquire,
        )
    }
}
