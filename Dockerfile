# latest stable rust build
FROM rust:1.63.0

# working directory app
# docker will create one if needed
WORKDIR /app
# install the required system dependencies
RUN apt update && apt install lld clang -y
#copy all files from our working environment
COPY . .
ENV SQLX_OFFLINE true
#build binary
RUN cargo build --release
# run release binary
ENTRYPOINT ["./target/release/zero2prod"]