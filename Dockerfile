# Dockerfile
FROM ubuntu:latest as builder

RUN apt-get update && \
    apt-get install -y curl gcc

WORKDIR /tmp

RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN chmod 755 rustup.sh
RUN ./rustup.sh -y
RUN rm /tmp/rustup.sh

RUN $HOME/.cargo/bin/cargo install mdbook
RUN $HOME/.cargo/bin/cargo install mdbook-katex

FROM debian:buster-slim

COPY --from=builder /root/.cargo/bin/mdbook /usr/local/bin/mdbook

WORKDIR /book

CMD ["mdbook", "serve", "--hostname", "0.0.0.0", "--port", "8899"]