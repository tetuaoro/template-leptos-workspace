cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
use crate::error::Result;
use once_cell::sync::Lazy;
use surrealdb::{engine::remote::ws::Client, Surreal};
type SC = Surreal<Client>;
static DB: Lazy<SC> = Lazy::new(Surreal::init);
}}

#[cfg(feature = "ssr")]
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
/// let client: Result<Surreal<_>, crate::error::Error> = get_db().await;
/// ```
///
/// [`Lazy`]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
pub async fn get_db() -> Result<&'static SC> {
    use std::{env, fs};
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::{Root, Scope};
    use surrealdb::sql::statements::{BeginStatement, CommitStatement};

    if let Ok(_health) = DB.health().await {
        log::debug!("DB ALREADY INIT");
        return Ok(&DB);
    }

    // Define constants for error messages and environment variable names
    const DB_NS: &str = "SURREAL_NS";
    const DB_DB: &str = "SURREAL_DB";
    const DB_ENDPOINT: &str = "SURREAL_BIND";
    const DB_R_USER: &str = "SURREAL_USER";
    const DB_R_PWD: &str = "SURREAL_PASS";
    const DB_DEFINITION_SCHEMA: &str = "SURREAL_DEFINITION_SCHEMA_PATH";

    let endpoint = env::var(DB_ENDPOINT)?;
    let namespace = env::var(DB_NS)?;
    let database = env::var(DB_DB)?;
    let username = env::var(DB_R_USER)?;
    let password = env::var(DB_R_PWD)?;

    DB.connect::<Ws>(endpoint).await?;
    DB.signin(Root {
        username: &username,
        password: &password,
    })
    .await?;
    DB.use_ns(&namespace).use_db(&database).await?;
    log::debug!("DB ROOT LOGIN SUCCESSFUL");

    if let Ok(definition_schema) = env::var(DB_DEFINITION_SCHEMA) {
        if let Ok(query) = fs::read_to_string(definition_schema) {
            // match error if schema already exists
            match DB
                .query(BeginStatement::default())
                .query(query)
                .query(CommitStatement::default())
                .await?
                .check()
            {
                Ok(_response) => log::debug!("DB SCHEMA SUCCESSFUL"),
                Err(msg) => log::error!("DB SCHEMA FAILDED : {msg}"),
            }
        }
    }

    // generate complex random string
    let pg = passwords::PasswordGenerator {
        length: 15,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };

    let username = pg.generate_one().unwrap();
    let password = pg.generate_one().unwrap();

    #[derive(Debug, serde::Serialize)]
    struct AuthParams {
        username: String,
        password: String,
        role: String,
    }
    let auth_user = AuthParams {
        username,
        password,
        role: "GUESS".to_owned(),
    };

    // connect with guess user
    DB.signup(Scope {
        namespace: &namespace,
        database: &database,
        scope: "account", // set the scope if needed
        params: auth_user,
    })
    .await?;
    log::debug!("DB GUESS USER LOGIN SUCCESSFUL");

    log::debug!("DB INITIALIZE SUCCESSFUL");
    Ok(&DB)
}
