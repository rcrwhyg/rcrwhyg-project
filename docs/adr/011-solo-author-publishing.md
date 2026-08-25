# ADR-011: Solo-author admin auth + publishing path

## Status

Accepted (login shell + CLI bootstrap + post CRUD)

## Context

A personal site needs a secure write path for its owner. Patterns are borrowed from the owner's prior Axum chat app (Argon2id), but transport is **server-side sessions in Postgres + HttpOnly cookies**, not long-lived Bearer JWT.

## Decisions

| Topic | Choice |
|-------|--------|
| Sessions | `admin_sessions` table; cookie holds raw token; DB stores SHA-256 hex |
| Cookie | `HttpOnly`, `SameSite=Strict`, `Path=/`; `Secure` when `COOKIE_SECURE=true` or `LEPTOS_ENV=PROD` |
| Admin bootstrap | **CLI only**: `create-admin` on the server shell (no public web setup) |
| Entry | `/admin` (+ `/admin/login`); not linked in public nav |
| Password | Argon2id; min length 12 |
| TLS | Provided by Caddy in production — app does not manage certs |
| Rate limit | **Public site** per-IP global limiter + **stricter auth** limiter (login) |
| Publishing | Authenticated CRUD under `/admin/posts` |

## Rate limiting

- Middleware on all Axum traffic (except `/health`)
- IP from `X-Forwarded-For` / `X-Real-IP` (Caddy) or peer addr
- Env: `RATE_LIMIT_PUBLIC_PER_MIN` (default 180), `RATE_LIMIT_AUTH_PER_MIN` (default 8)
- Auth server fns also check the auth limiter

## Schema

See `sql/auth.sql` (`admins`, `admin_sessions`).

## Bootstrap (ops)

```bash
# After sql/auth.sql is applied, on the machine that can reach DATABASE_URL:
cargo run --features ssr --bin create-admin -- you@example.com 'at-least-12-chars'
# production: ship the create-admin binary (or run once via SSH against the same DB)
```

Fails if an admin already exists. Password never crosses a public bootstrap form.
