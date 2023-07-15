set export
DATABASE_URL := "postgresql://user:pass@localhost:5432/local-db"
RUST_LOG := "debug=sea_orm"
RUST_BACKTRACE := "1"

start:
      cd migration && cargo build
      docker compose down || true
      docker compose up -d db
      @docker compose run --rm wait_for_db
      cd migration && cargo run -- up
      @echo "database is now ready at $DATABASE_URL"

test: start
      cd migration && cargo test
      docker compose down
