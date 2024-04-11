FROM rust:latest

WORKDIR /app

COPY . .

COPY Cargo.toml ./
RUN cargo build --release

CMD ["./target/release/server"]