FROM rust:1.86 AS builder
WORKDIR /usr/src/sfls
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libpq-dev libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sfls /usr/local/bin/sfls
RUN useradd -ms /bin/bash sfls
USER sfls
WORKDIR /home/sfls
CMD ["sfls"]
