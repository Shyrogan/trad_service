FROM rust:1.75

WORKDIR /usr/src/trad_service
COPY . .

RUN cargo install --path .

CMD ["trad_service"]

