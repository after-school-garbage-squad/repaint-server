# bullseye is the codename for Debian 11
FROM rust:1.72-slim-bullseye as base

ENV CARGO_TERM_PROGRESS_WHEN="always" \
    CARGO_TERM_PROGRESS_WIDTH="80"
WORKDIR /app

COPY rust-toolchain.toml .
RUN rustup toolchain install stable \
    && cargo +stable install cargo-chef --locked

FROM base as plan

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as build

RUN apt-get update \
    && apt-get install -y git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=plan /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN GIT_HASH=`git rev-parse HEAD | head -c 7` \
    cargo build --release

FROM gcr.io/distroless/cc-debian11 as runtime

LABEL org.opencontainers.image.source=https://github.com/after-school-garbage-squad/repaint-server
USER nonroot:nonroot
COPY --chown=nonroot:nonroot --from=build /app/target/release/repaint-server /

ARG SA_KEY=""
RUN echo "$SA_KEY" > /app/credentials.json.json

ENTRYPOINT [ "/repaint-server" ]
