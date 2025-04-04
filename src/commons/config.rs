use std::sync::LazyLock;

pub static JWT_EXPIRATION_SECONDS: LazyLock<i64> = LazyLock::new(|| {
    std::env::var("JWT_EXPIRATION_SECONDS")
        .unwrap_or_else(|_| "3600".to_string())
        .parse()
        .unwrap()
});

pub static MAX_MISSMATCH_COUNT: LazyLock<i32> = LazyLock::new(|| {
    std::env::var("MAX_MISSMATCH_COUNT")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .unwrap()
});

pub static DB_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("DB_URL").unwrap_or_else(|_| "sqlite:./data/database.db".to_string())
});

pub static HOST_NAME: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOST_NAME").unwrap_or_else(|_| "0.0.0.0:3000".to_string()));
