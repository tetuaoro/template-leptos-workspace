<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework, the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum), the multi-model database [Surrealdb](https://github.com/surrealdb/surrealdb/) and the awesome component library web, [Leptonic](https://github.com/lpotthast/leptonic).

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```

Then run
```bash
cargo leptos new -n {projectname} --git https://github.com/tetuaoro/template-leptos-workspace/
```

to generate a new project template.

```bash
cd {projectname}
```

to go to your newly created project.  
Feel free to explore the project structure, but the best place to start with your application code is in `src/app.rs`.  
Addtionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

### Islands support

Note that for islands to work correctly, you need to have a `use app;` in your frontend `lib.rs` otherwise rustc / wasm_bindgen gets confused.
To prevent clippy from complaining, at the top of the `frontend/lib.rs` file place:
```rust
#[allow(unused_imports)]
use app;
```

## Surrealdb setup
### Environment file

If you use `surrealdb` crate for your database, you need to create `.env` file  at the root directory of the project with the following content :

```
SURREAL_NS=""
SURREAL_DB=""
SURREAL_BIND=""
SURREAL_USER=""
SURREAL_PASS=""
SURREAL_DEFINITION_SCHEMA_PATH=""
```

Replace the empty strings with the appropriate values for your project. All the variables are required while `SURREAL_DEFINITION_SCHEMA_PATH` is optional.

*N.B When your app is running, the instance runs with a random guess [scope user](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/scope#example-usage) to ensure security and permission access.*

#### SURREALDB DEFINITION

A starter template file `SURREALDB_DEFINITION` is define in the root project.

### Usage

To use the `surrealdb` crate for database operations, you can use the `get_db()` function provided. To initialize database at runtime, uncomment the main server function :

```rust
# server/src/main.rs
// services::get_db().await.expect("couldn't intialize database");
```

Here's an example of how to use the `get_db()` function :

```rust
use services::get_db;
...
// somewhere inside an async fn
let client: Result<Surreal<_>, crate::error::Error> = db::get_db().await;
...
```

In this example, the `get_db()` function is called inside an asynchronous function to obtain an instance of the database client. This instance can then be used for database operations.

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
start-axum
site/
```
Set the following enviornment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="start-axum"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
