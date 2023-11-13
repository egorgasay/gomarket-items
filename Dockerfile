# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

FROM base AS development

EXPOSE 8000

CMD [ "diesel", "migration", "run", "--database-url=postgresql://postgres:1234@db:5432/postgres", "&&", "cargo", "run"]

FROM base AS builder

RUN cargo build

FROM debian:buster-slim

ENV DATABASE_URL=postgresql://postgres:1234@127.0.0.1:5432/postgres

EXPOSE 8000

RUN apt-get update && apt-get install -y libpq5

COPY --from=builder /code/target/debug/gomarket-items /gomarket-items

CMD [ "/gomarket-items" ]