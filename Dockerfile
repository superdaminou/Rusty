FROM rust:1.67.0

WORKDIR /src/app

COPY entrypoint.sh .
ENTRYPOINT ./entrypoint.sh


