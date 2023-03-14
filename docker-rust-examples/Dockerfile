FROM rust:latest as builder

RUN USER=root cargo new --bin docker-rust-examples
WORKDIR ./docker-rust-examples
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN ls -l ./target/release/deps/
RUN rm ./target/release/deps/docker_rust_examples*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC APP_USER=appuser

RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER && mkdir -p ${APP}

COPY --from=builder /docker-rust-examples/target/release/docker-rust-examples ${APP}/docker-rust-examples

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENV LOG_LEVEL=debug
CMD ["./docker-rust-examples"]
