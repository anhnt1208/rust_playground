FROM rust:alpine3.20

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["rust_playground"]
