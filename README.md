# kprsnt.in — Rust + Actix-web Version

Portfolio website built with **Rust + Actix-web**, featuring the same design as the Astro and Laravel versions.

## Tech Stack
- **Backend**: Rust, Actix-web 4 (one of the fastest web frameworks)
- **Templates**: Tera (Jinja2-like syntax)
- **Styling**: Bootstrap Darkly + custom CSS (glassmorphism)
- **AI**: Gemini API via reqwest HTTP client

## Why Rust?
- **Blazing fast** — sub-millisecond response times
- **Single binary** — no runtime dependencies
- **Memory safe** — no garbage collector, zero-cost abstractions
- **Actix-web** — consistently top performer in TechEmpower benchmarks

## Local Setup

```bash
# Build and run
cargo run

# Or build optimized release
cargo build --release
./target/release/kprsnt-portfolio

# Server runs at http://localhost:8080
```

## Environment Variables

| Variable | Description |
|---|---|
| `PORT` | Server port (default: 8080) |
| `GEMINI_API_KEY` | Google Gemini API key for AI Insight |

## Deploy to Vercel

> **Note**: Vercel doesn't natively run Rust servers. Best deployment options:

### Option A: Railway (Recommended)

1. Connect your GitHub repo at [railway.app](https://railway.app)
2. Railway auto-detects Rust and builds with `cargo build --release`
3. Set environment variables: `PORT`, `GEMINI_API_KEY`
4. Push and deploy — Railway handles the rest

### Option B: Shuttle.rs (Rust-native hosting)

1. Install Shuttle CLI: `cargo install cargo-shuttle`
2. Deploy: `cargo shuttle deploy`

### Option C: Fly.io

1. Install flyctl: `curl -L https://fly.io/install.sh | sh`
2. Create a `Dockerfile`:
```dockerfile
FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/kprsnt-portfolio /usr/local/bin/
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
WORKDIR /app
CMD ["kprsnt-portfolio"]
```
3. Deploy: `fly launch && fly deploy`

### Option D: Docker (any VPS)

```bash
docker build -t portfolio-rust .
docker run -p 8080:8080 -e GEMINI_API_KEY=your-key portfolio-rust
```

## Pages
- `/` — About Me (home)
- `/skills` — Technical Skills
- `/projects` — Project Portfolio
- `/resume` — Resume / CV
- `/blog` — Blog listing
- `/blog/{slug}` — Blog post
- `/plotter` — Interactive Data Plotter
- `POST /api/ai-insight` — AI Insight API

## Performance
Actix-web is one of the fastest web frameworks available. Expected response times:
- Static pages: **< 1ms**
- AI Insight API: **1-3s** (waiting for Gemini API)
