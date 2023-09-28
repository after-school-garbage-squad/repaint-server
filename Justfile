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

run: start
      cargo run

gen: start
      mkdir -p core/src/entity && rm -rf core/src/entity/*.rs && rm -rf core/src/entity.rs
      sea-orm-cli generate entity -o core/src/entity
      patch -p1 --no-backup-if-mismatch < core/patch/entity.patch
      cp core/src/entity/mod.rs core/src/entity.rs && rm -rf core/src/entity/mod.rs
      docker compose down

test: start
      cd migration && cargo test
      cd model && cargo test
      cd core && cargo test
      docker compose down

license:
      cargo about generate about.hbs > api/asset/license.html
