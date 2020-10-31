FROM rust:1.47

WORKDIR /usr/src/gorp_simple_miner
COPY . .

RUN cargo install --path .

CMD ["gorp_simple_miner"]