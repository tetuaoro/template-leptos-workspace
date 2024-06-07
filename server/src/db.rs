use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    err::Error,
    Result, Surreal,
};

type SC = Surreal<Client>;
static DB: Lazy<SC> = Lazy::new(Surreal::init);

// Define constants for error messages and environment variable names
const MISS_ENV_FILE: &str = "MISSING .ENV FILE";
const MISS_DB_NS: &str = "MISSING SURREALDB NAMESPACE PARAMS";
const MISS_DB_DB: &str = "MISSING SURREALDB DATABASE PARAMS";
const DB_NS: &str = "SURREALDB_NS";
const DB_DB: &str = "SURREALDB_DB";
const DB_EPT: &str = "SURREALDB_ENDPOINT";
const DB_EPT_DEFAULT: &str = "127.0.0.1:8000";

/// Define an asynchronous function to get the database instance
///
/// This function checks if the database is healthy. If it is, it returns a clone of the database instance.
/// If the database is not healthy, it loads the environment variables from the .env file, connects to the database
/// using the WebSocket protocol, sets the namespace and database to use, and returns a clone of the database instance.
///
/// The [`Lazy`] crate is used to ensure that the database instance is created only once, even if the function is called
/// multiple times. This is done by creating a static instance of `Surreal<Client>` and using the `Lazy::new` function
/// to initialize it. The `get_db` function then returns a clone of this static instance.
///
/// ### Get database client
///
/// ```rust
/// let client: Result<Surreal<_>, surrealdb::Error> = get_db().await;
/// ```
///
/// [`Lazy`]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
pub async fn get_db() -> Result<SC> {
    if let Ok(_health) = DB.health().await {
        return Ok(DB.clone());
    }

    dotenv().map_err(|msg| Error::InvalidArguments {
        name: MISS_ENV_FILE.to_owned(),
        message: msg.to_string(),
    })?;

    let ept_default = DB_EPT_DEFAULT.to_string();
    let endpoint = env::var(DB_EPT).map_or(ept_default.clone(), |value| {
        if value.is_empty() {
            return ept_default;
        }
        value
    });
    let namespace = env::var(DB_NS).map_err(|msg| Error::InvalidArguments {
        name: MISS_DB_NS.to_owned(),
        message: msg.to_string(),
    })?;
    let database = env::var(DB_DB).map_err(|msg| Error::InvalidArguments {
        name: MISS_DB_DB.to_owned(),
        message: msg.to_string(),
    })?;

    DB.connect::<Ws>(endpoint).await?;
    DB.use_ns(namespace).use_db(database).await?;

    Ok(DB.clone())
}
