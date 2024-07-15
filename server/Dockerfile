FROM rust:1.79-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

from chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build deps
RUN cargo chef cook --release --recipe-path recipe.json
# Build app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as runner
WORKDIR /app
COPY --from=builder /app/target/release/grooves-server /bin
ENTRYPOINT ["/bin/grooves-server"]
