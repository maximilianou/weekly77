FROM rust:1.70-slim
WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev libsqlite3-dev gcc curl && rm -rf /var/lib/apt/lists/*
RUN cargo build --release
EXPOSE 3001
CMD ["/usr/local/cargo/bin/cargo", "run", "--release"]
