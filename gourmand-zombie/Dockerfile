FROM rust

WORKDIR /

RUN USER=root cargo new proj

WORKDIR /proj

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

COPY ./src ./src
COPY .env .
COPY ./migrations ./migrations
RUN cargo build --release

EXPOSE 1420
CMD ["./target/release/gourmand-zombie"]
