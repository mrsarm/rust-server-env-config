server-env-config
=================

Util types and functions to quickly and easy set up an HTTP server from
environment variables (although you can omit env variables).

Once you have the Rust objects with all the basic information you need for your server,
like the database connection (`DATABASE_URL`), the deployment environment (`APP_ENV`)
or the port where to start the app (`PORT`), you can use the struct objects
to use those values to start up your Actix server, Rocket server, or whatever
server your app use.

Check the 📖 docs at https://docs.rs/server-env-config/.

### Examples

```rust
use std::env;
use server_env_config::Config;
use server_env_config::env::Environment;

// Configurations should be actually set by the OS environment
env::set_var("APP_ENV", "production");  // if not set, "local" is the default
env::set_var("APP_URI", "api/v1");
env::set_var("PORT", "8080");
env::set_var("DATABASE_URL", "postgresql://user:pass@localhost/db");

let result = Config::init(9999);        // 9999 will be used if "PORT" is not set
assert!(result.is_ok());
let config = result.unwrap();
assert_eq!(config.env, Environment::Production);
assert_eq!(config.server.port, 8080);
assert_eq!(config.server.url, "http://127.0.0.1:8080/api/v1/");    // calculated field
assert_eq!(config.db.database_url, "postgresql://user:pass@localhost/db");
// Some settings have default values if env variables are not set
assert_eq!(config.db.min_connections, 1);
assert_eq!(config.db.max_connections, 10);
// The `to_string()` method prints out all variables in .env format
println!("{}", config.to_string());
// # APP_URL --> http://127.0.0.1:8080/api/v1/
// APP_URI="api/v1"
// HOST=127.0.0.1
// PORT=8080
// APP_ENV=production
// DATABASE_URL="postgresql://user:pass@localhost/db"
// MIN_CONNECTIONS=1
```

### About

**Project Home**: https://github.com/mrsarm/rust-actix-contrib-rest.

#### Authors

- Mariano Ruiz (mrsarm at gmail.com).

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])
