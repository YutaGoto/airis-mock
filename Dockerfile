FROM rust:1.87.0

WORKDIR /app

COPY . .

RUN rustup install nightly
RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]
