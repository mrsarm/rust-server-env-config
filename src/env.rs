//! The [`Environment`] enum represents possible deployment environments for an application.

use anyhow::{Context, Result};
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Possible deployment environments for an application.
#[derive(Debug, Default, Display, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
    #[default]
    Local,
    Test,
    Stage,
    Production,
}

impl Environment {
    /// Get the value from the environment variable `APP_ENV`.
    /// It raise an error if the string doesn't match a possible environment.
    /// # Examples
    /// ```
    /// use std::env;
    /// use server_env_config::env::Environment;
    ///
    /// env::set_var("APP_ENV", "production");
    /// assert!(matches!(Environment::init(), Ok(Environment::Production)));
    /// env::set_var("APP_ENV", "Not a environment");
    /// assert!(Environment::init().is_err());
    /// env::remove_var("APP_ENV"); // if not set, local environment is the default
    /// assert!(matches!(Environment::init(), Ok(Environment::Local)));
    /// ```
    pub fn init() -> Result<Self> {
        let app_env = env::var("APP_ENV");
        match app_env {
            Err(_) => Ok(Environment::default()),
            Ok(env) => Environment::from_str(env.as_str())
                .with_context(|| format!("APP_ENV invalid value \"{env}\"")),
        }
    }
}
