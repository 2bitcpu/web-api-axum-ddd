use crate::commons::types::{BoxError, DbPool};
use sqlx::migrate::MigrateDatabase;

pub async fn initialize_db(db_url: &str) -> Result<DbPool, BoxError> {
    if db_url != "sqlite::memory:" {
        if !sqlx::Sqlite::database_exists(db_url).await? {
            sqlx::Sqlite::create_database(db_url).await?;
        }
    }

    let pool: DbPool = sqlx::SqlitePool::connect(db_url).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS member (
            account VARCHAR(32) NOT NULL PRIMARY KEY,
            password VARCHAR(512) NOT NULL,
            name VARCHAR(64),
            email VARCHAR(256),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS auth (
            account VARCHAR(32) NOT NULL PRIMARY KEY,
            issued_tm INTEGER,
            expired_tm INTEGER,
            jwt_id VARCHAR(256),
            missmatch INTEGER,
            login_at DATETIME,
            prev_login_at DATETIME
        );",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS content (
            content_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            account VARCHAR(32) NOT NULL,
            post_at DATETIME NOT NULL,
            title VARCHAR(256) NOT NULL,
            body TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
