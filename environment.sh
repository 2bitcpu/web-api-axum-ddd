#/bin/bash -x

cargo init --name web-api

cargo add tokio --features macros,rt-multi-thread --no-default-features
cargo add serde --features derive --no-default-features
cargo add serde_json --features std --no-default-features
cargo add chrono --features serde,now --no-default-features
cargo add async-trait --no-default-features
cargo add sqlx --features runtime-tokio-native-tls,chrono,derive --no-default-features
cargo add axum
cargo add axum-extra --features typed-header --no-default-features
cargo add derive-new --no-default-features

cat << EOS >> Cargo.toml

simple-jwt = { git = "https://github.com/2bitcpu/simple-jwt" }
async-argon2 = { git = "https://github.com/2bitcpu/async-argon2" }

[features]
default = [ "sqlite" ]
sqlite = [ "sqlx/sqlite-unbundled" ]
sqlite-bundled = [ "sqlx/sqlite" ]

[profile.release]
opt-level = "z"
debug = false
lto = true
strip = true
codegen-units = 1
panic = "abort"

# cargo +nightly-2025-02-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target aarch64-unknown-linux-gnu --release
# upx --best --lzma ./target/aarch64-unknown-linux-gnu/release/web-api
EOS

touch src/lib.rs

mkdir src/commons
touch src/commons/types.rs
touch src/commons/config.rs
touch src/commons.rs

echo "pub mod types;" >> src/commons.rs
echo "pub mod config;" >> src/commons.rs
echo "pub mod commons;" >> src/lib.rs

mkdir src/models
mkdir src/models/dtos
mkdir src/models/entities

touch src/models/dtos.rs
touch src/models/entities.rs
touch src/models.rs

echo "pub mod dtos;" >> src/models.rs
echo "pub mod entities;" >> src/models.rs
echo "pub mod models;" >> src/lib.rs

mkdir src/repositories
mkdir src/repositories/interfaces
mkdir src/repositories/implementations

touch src/repositories/interfaces.rs
touch src/repositories/implementations.rs
touch src/repositories.rs

echo "pub mod interfaces;" >> src/repositories.rs
echo "pub mod implementations;" >> src/repositories.rs
echo "pub mod repositories;" >> src/lib.rs

mkdir src/use_cases
touch src/use_cases.rs

echo "pub mod use_cases;" >> src/lib.rs

mkdir src/middlewares
touch src/middlewares.rs

echo "pub mod middlewares;" >> src/lib.rs

mkdir src/handlers
touch src/handlers.rs

echo "pub mod handlers;" >> src/lib.rs
