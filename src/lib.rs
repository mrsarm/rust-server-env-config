//! Util types and functions to quickly and easy set up an HTTP server from environment variables.
//!
//! See [`Config::init()`] for examples.

mod conf;
pub mod db;
pub mod env;
pub mod server;

use anyhow::{anyhow, Context, Result};
use std::env::var;
use std::fmt::Debug;
use std::str::FromStr;

pub use self::conf::Config;

/// Read boolean environment variable, accepting "0" or "false" as false
/// values, and "1" or "true" values as true.
/// # Examples
/// ```
/// use std::env;
/// use server_env_config::env_bool;
///
/// // Right values
/// env::set_var("BOOL_ENV", "true");
/// assert!(matches!(env_bool("BOOL_ENV", false), Ok(true)));
/// env::set_var("BOOL_ENV", "0");
/// assert!(matches!(env_bool("BOOL_ENV", true), Ok(false)));
///
/// // No value set
/// assert!(matches!(env_bool("NOT_SET_ENV", true), Ok(true)));
///
/// // Wrong value
/// env::set_var("BOOL_ENV", "not a boolean");
/// assert!(env_bool("BOOL_ENV", false).is_err());
/// ```
pub fn env_bool(env_name: &'static str, default_value: bool) -> Result<bool> {
    var(env_name)
        .map(|v| match v.as_str() {
            "0" => "false".to_owned(),
            "1" => "true".to_owned(),
            _ => v.to_lowercase(),
        })
        .map(|v| {
            v.parse::<bool>()
                .with_context(|| format!("{env_name} invalid boolean \"{v}\""))
        })
        .unwrap_or(Ok(default_value))
}

/// Get a parsable value from an env value like a number,
/// otherwise return `default_value`.
/// # Examples
/// ```
/// use std::env;
/// use server_env_config::env_parsable;
///
/// // Right values
/// env::set_var("NUM_ENV", "1234");
/// assert!(matches!(env_parsable::<u16>("NUM_ENV", 1), Ok(1234)));
///
/// // No value set
/// assert!(matches!(env_parsable::<u32>("ENV_NOT_SET", 1), Ok(1)));
///
/// // Wrong value
/// env::set_var("LONG_ENV", "not a number");
/// assert!(env_parsable::<i64>("LONG_ENV", 1).is_err());
/// ```
pub fn env_parsable<A: FromStr>(env_name: &'static str, default_value: A) -> Result<A>
where
    <A as FromStr>::Err: Debug,
{
    var(env_name)
        .map(|v| {
            v.parse::<A>()
                .map_err(|_| anyhow!("{env_name} invalid number \"{v}\""))
        })
        .unwrap_or(Ok(default_value))
}
