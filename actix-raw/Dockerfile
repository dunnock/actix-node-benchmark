FROM rust:1.41

WORKDIR /app
COPY . .

RUN cargo build --release

CMD cargo run --release