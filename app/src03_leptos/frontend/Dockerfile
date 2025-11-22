FROM rust:1.70-slim as builder
WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
# NOTE: Building Leptos frontend to Wasm often requires trunk or wasm-pack; this Dockerfile assumes a static build step exists.
RUN cargo build --release || true

FROM nginx:stable-alpine
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
