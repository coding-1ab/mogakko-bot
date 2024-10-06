FROM rust:1.81.0 AS builder

WORKDIR /app

RUN ["rustup", "target", "add", "x86_64-unknown-linux-musl"]
RUN ["apt", "update"]
RUN ["apt", "install", "-y", "musl-tools", "musl-dev"]
RUN ["update-ca-certificates"]

RUN ["cargo", "install", "sqlx-cli", "--no-default-features", "--features", "native-tls,sqlite"]

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

RUN ["sqlx", "database", "create"]

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./migrations/ ./migrations/

RUN ["sqlx", "migrate", "run"]

COPY ./src/ ./src/

RUN ["cargo", "install", "--path", ".", "--target", "x86_64-unknown-linux-musl"]
FROM alpine:latest AS runner

WORKDIR /app/data

COPY --from=builder /usr/local/cargo/bin/mogakko-bot /usr/local/bin/mogakko-bot
COPY --from=builder /app/mogakko.db /app/mogakko.db
COPY ./init.sh /app

CMD ../init.sh && mogakko-bot
