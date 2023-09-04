FROM rust:1.67

WORKDIR /usr/src/rusty
COPY . .

RUN cargo install --path .

CMD ["rusty"]