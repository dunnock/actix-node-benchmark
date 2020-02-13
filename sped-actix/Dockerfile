FROM rust:1.41

WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/bin/stub.rs ./src/bin/stub.rs

RUN cargo build --release --bin stub

COPY . .

RUN cargo build --release --bin server

CMD ["cargo run --release --bin server"]