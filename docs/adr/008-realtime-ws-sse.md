# ADR-008: Axum HTTP + WebSocket + SSE

## Status

Accepted

## Context

Axum will host more than Leptos HTML routes. Tools and future features need realtime channels (live updates, streams, interactive widgets).

## Decision

- One Axum application process serves:
  - Leptos SSR routes
  - HTTP APIs
  - WebSocket endpoints
  - Server-Sent Events (SSE) endpoints
- Nest realtime routes under clear prefixes (`/ws/...`, `/sse/...`)
- Share `AppState` (DB pool, broadcast channels, etc.) across handlers
- Prefer Tokio broadcast / watch channels for fan-out; keep handlers non-blocking
- Leptos server functions remain for request/response RPC; use WS/SSE when push or bidirectional streams are required
- Optionally use Leptos `server_fn` websocket codecs where they fit; otherwise raw Axum WS is fine

## Consequences

- Global `rust-axum-tokio` skill covers WS/SSE patterns
- Feature-gate `axum` websocket features (`ws`) when enabled
- musl builds must keep TLS/crypto crates zigbuild-friendly (`rustls`)
- Current stubs: `GET /sse/heartbeat`, `GET /ws/echo`, `GET /health`
- Optional `PgPool` lives on `AppState` and is injected via `leptos_routes_with_context` when `DATABASE_URL` is set
