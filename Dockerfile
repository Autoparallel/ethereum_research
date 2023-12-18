# Dockerfile
FROM rust:1.54 as builder

WORKDIR /usr/src

RUN cargo install mdbook

RUN cargo install mdbook-katex

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/mdbook /usr/local/bin/mdbook

WORKDIR /book

CMD ["mdbook", "serve", "--hostname", "0.0.0.0", "--port", "8899"]