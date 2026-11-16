FROM rust:1.100.0

WORKDIR /app

COPY . .

RUN rustup install nightly
RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]
