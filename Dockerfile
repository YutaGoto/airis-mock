FROM rust:1.98.1

WORKDIR /app

COPY . .

RUN rustup install nightly
RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]
