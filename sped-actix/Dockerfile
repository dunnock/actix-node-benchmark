FROM rust:1.41

WORKDIR /app
COPY . .

RUN cargo build --release --bin server

CMD cargo run --release --bin server