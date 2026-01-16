FROM ubuntu:24.04

WORKDIR /client

RUN apt update && apt install -y gcc rustup pkg-config libssl-dev curl
RUN rustup default stable
ADD ServiceAPIClient /client/ServiceAPIClient
ADD test-api /client/test-api

WORKDIR /client/test-api

RUN cargo build

CMD ./run.sh