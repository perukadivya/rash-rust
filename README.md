# kprsnt.in — Rust + Actix-web Version

Portfolio website built with **Rust + Actix-web** — blazing fast server-side rendering.

## Tech Stack
- **Backend**: Rust, Actix-web 4
- **Templates**: Tera (Jinja2-like syntax)
- **Styling**: Bootstrap Darkly + custom CSS (glassmorphism)
- **AI**: Gemini API via reqwest HTTP client

## Local Setup
```bash
cargo run                    # http://localhost:8080
cargo build --release        # Optimized binary
```

## Deploy to Cloudflare

### Option A: Cloudflare Tunnel + VPS (Recommended)

Run the Rust binary on a VPS and expose via Cloudflare Tunnel:

```bash
# Build on VPS
cargo build --release

# Install cloudflared
curl -L https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64 -o /usr/local/bin/cloudflared
chmod +x /usr/local/bin/cloudflared

# Create tunnel
cloudflared tunnel login
cloudflared tunnel create rust-portfolio
cloudflared tunnel route dns rust-portfolio your-domain.com

# Run
GEMINI_API_KEY=your-key PORT=8080 ./target/release/kprsnt-portfolio &
cloudflared tunnel run --url http://localhost:8080 rust-portfolio
```

### Option B: Railway (Easiest)

1. Go to [railway.app](https://railway.app) → New Project → Deploy from GitHub
2. Select `rash-rust` repo
3. Railway auto-detects Rust and runs `cargo build --release`
4. Set env: `PORT=8080`, `GEMINI_API_KEY`
5. Point Cloudflare DNS to Railway's URL

### Option C: Docker + Fly.io

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
ENV PORT=8080
CMD ["kprsnt-portfolio"]
```

```bash
fly launch && fly deploy
```

Then point Cloudflare DNS to your Fly.io URL.

## Environment Variables

| Variable | Description |
|---|---|
| `PORT` | Server port (default: 8080) |
| `GEMINI_API_KEY` | Google Gemini API key |

## Pages
- `/` — About Me
- `/skills` — Technical Skills
- `/projects` — Project Portfolio
- `/resume` — Resume
- `/blog` — Blog listing
- `/blog/{slug}` — Blog post
- `/plotter` — Data Plotter
- `POST /api/ai-insight` — AI Insight API

## Performance
Actix-web delivers sub-millisecond response times for all pages.
