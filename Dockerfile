
#prepare caching
FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm as chef
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# dependency builder
FROM chef AS builder

COPY --from=planner /app/ .
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && cp cargo-binstall /usr/local/cargo/bin \
  && cargo binstall cargo-leptos -y

RUN cargo leptos build --release --workspace -vv

# Runtime build
FROM debian:bookworm-slim as runtime
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/.env /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 3000
EXPOSE 3443

CMD ["/app/server"]
