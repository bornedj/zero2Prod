#using the cargo-chef image to cache dependecies
FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
# working directory app
# docker will create one if needed
WORKDIR /app
# install the required system dependencies
RUN apt update && apt install lld clang -y
#copy all files from our working environment

FROM chef as planner
COPY . .
# compute the lock file for dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# all layers cached

COPY . .
ENV SQLX_OFFLINE true
#build project
RUN cargo build --release --bin zero2prod
# run release binary

EXPOSE 8000

#runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
# Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./zero2prod" ]