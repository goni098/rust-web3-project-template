# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build entire workspace
cargo build --locked --release --workspace

# Run individual services
cargo run -p http-server
cargo run -p evm-scanner -- <chain_id>   # e.g., 1 for Ethereum, 42161 for Arbitrum
cargo run -p evm-stream -- <chain_id>
cargo run -p solana-scanner
cargo run -p solana-stream

# Database schema
pnpm db:push        # Push Prisma schema to PostgreSQL
pnpm seagen         # Regenerate SeaORM entities from database

# Run a single test
cargo test -p <crate-name> <test_name>
```

## Architecture

This is a modular blockchain event monitoring system with dual support for EVM and Solana networks. Each chain has two monitoring modes: polling (scanner) and real-time WebSocket (stream).

### Workspace Layout

- **`crates/`** — Shared infrastructure: `shared`, `database`, `http-server`, `ws-client`
- **`evm/`** — EVM chain modules: `lib` (Alloy client), `scanner` (polling), `stream` (WebSocket)
- **`solana/`** — Solana modules: `lib` (Anchor parser), `scanner` (polling), `stream` (WebSocket)

### Shared Infrastructure (`crates/`)

**`crates/shared`** provides cross-cutting concerns:
- `Env` enum for typed environment variable access (supports chain-specific RPC via `PUBLIC_RPC_CHAIN_<id>`, `WS_RPC_CHAIN_<id>`)
- `Rs<T>` = `Result<T, TracedAppErr>` — custom error type that captures `SpanTrace` for diagnostics
- CLI argument parsing (chain ID from process args)

**`crates/database`** (SeaORM + PostgreSQL):
- Entities: `user`, `signature`, `setting`
- `Setting` key/value store tracks scanner progress: `evm_scanned_block_chain_<id>`, `solana_current_scanned_signature`
- Scanners read/write these keys to resume after restart

**`crates/http-server`** (Axum, port 8080):
- Routes: `GET /`, `GET /docs/openapi.yml`, `GET /docs`, `GET /random-u64` (WebSocket demo), user auth routes
- JWT auth middleware; CORS permissive
- AppState uses Axum's `FromRef` for dependency injection

**`crates/ws-client`**: TLS WebSocket client with `FrameCollector` for reassembling fragmented frames.

### EVM Monitoring (`evm/`)

**`evm/lib`**: Alloy-based client with fallback RPC (private → public), EIP-1559 support, gas estimation with buffer, and retry (3 attempts, 2s delay). Decodes Uniswap V2 and V3 pool events from specific mainnet contract addresses.

**`evm/scanner`**: Fetches logs from configured Uniswap pools in ~2,000-block windows, stores next block height in DB `setting`, polls every 60s.

**`evm/stream`**: Subscribes to `eth_subscribe` via JSON-RPC 2.0 WebSocket. Pings every 30s, reconnects after 3s on disconnect.

### Solana Monitoring (`solana/`)

**`solana/lib`**: Parses Pumpfun program events using `anchor-parser`.

**`solana/scanner`**: Cursor-based pagination over transaction signatures for the Pumpfun program. Stores last processed signature in DB. Processes 30 signatures concurrently, polls every 6s.

**`solana/stream`**: Subscribes to `logsSubscribe` filtered by Pumpfun program. Pings every 30s; reconnects after detecting zombie connections (no messages for 300s).

### Error Handling Pattern

```
AppErr (enum: I/O, DB, RPC, Solana, custom)
  → TracedAppErr (wraps with SpanTrace)
  → Rs<T> = Result<T, TracedAppErr>
```

All async functions return `Rs<T>`.

### Environment Variables

See `.env.example`. Chain-specific RPC endpoints follow the pattern:
- `PUBLIC_RPC_CHAIN_<id>` — public HTTP RPC
- `PRIVATE_RPC_CHAIN_<id>` — private/paid HTTP RPC (EVM client prefers this)
- `WS_RPC_CHAIN_<id>` — WebSocket RPC for streaming

### Deployment

CI/CD (`.github/workflows/main.yml`) triggers on `dev` branch push: builds release binary, SCPs artifacts to VPS, runs `exe.sh` via SSH. PM2 (`ecosystem.config.js`) manages the `http-server` process in production.
