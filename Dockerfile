FROM rust:1.59 as build

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# build for release
RUN rm /app/target/release/deps/app*
RUN cargo build --release

#Second stage
FROM rust:1.59-slim-buster

COPY --from=build /app/target/release/app .

CMD ["./app"]