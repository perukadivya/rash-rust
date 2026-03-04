FROM rust:1.85 as builder
WORKDIR /app

# Install OpenSSL dev headers (required by reqwest's native-tls)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependency builds
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Build the actual app
COPY . .
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/kprsnt-portfolio /usr/local/bin/
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
WORKDIR /app
ENV PORT=8080
EXPOSE 8080
CMD ["kprsnt-portfolio"]
