use diesel::prelude::*;
use diesel_migrations::{
    embed_migrations,
    EmbeddedMigrations,
    MigrationHarness
};
use dotenvy::dotenv;
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn setup_database() -> SqliteConnection {
    let mut conn = setup_conn();

    migrate(&mut conn);

    conn
}

fn setup_conn() -> SqliteConnection {
    dotenv().ok();
    let xdg_dirs = xdg::BaseDirectories::new()
        .expect("Cannot open base xdg directory");

    let path = xdg_dirs.create_data_directory("quiz")
        .expect("Cannot create data directory");

    let database_url = path.join("database.db");

    let database_url = database_url.to_str()
        .expect("Invalid utf8 in database path");

    let conn = SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    conn
}

fn migrate(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
