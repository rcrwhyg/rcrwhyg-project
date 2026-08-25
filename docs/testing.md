# Testing — 如春日午后阳光

## Current coverage (honest baseline)

| Layer | Status | Notes |
|-------|--------|--------|
| Domain unit | **Thin** | Markdown export roundtrip; seed post shape |
| Server unit | **Thin** | Markdown→HTML; `PostSummary` from seed |
| DB integration | **Soft-gated** | Uses `.env` `DATABASE_URL` + shared `PgPool`; skips if unavailable |
| Playwright e2e | **Stale scaffold** | `end2end/tests/example.spec.ts` still expects old Leptos welcome copy — update before relying on it |
| Coverage % tooling | **Not enforced** | No `cargo-llvm-cov` / tarpaulin gate yet |

Do **not** claim high coverage. Prefer adding tests with each feature (posts write path, auth, tools).

## Pyramid (what to write)

1. **Unit (default, always run)** — pure domain / rendering helpers; no network.
2. **Integration (local Postgres)** — same `DATABASE_URL` as `cargo leptos watch`; share one pool per test process (`src/server/test_db.rs`).
3. **E2E (optional)** — Playwright via `cargo leptos end-to-end` after fixing selectors/titles.

## Commands

```bash
# unit + soft DB integration (ssr)
cargo test --features ssr

# hydrate still has no runtime tests; at least typecheck
cargo check --lib --features hydrate --target wasm32-unknown-unknown

# e2e (after updating specs): start site, then
# cargo leptos end-to-end
```

Integration tests that need schema:

```bash
psql "$DATABASE_URL" -f sql/posts.sql -f sql/seed_posts.sql
cargo test --features ssr
```

If `DATABASE_URL` is missing or connect fails, DB tests **print skip and return** (do not fail CI solely for that). Schema/query errors after a successful connect should still fail.

## Shared pool convention

- Load `.env` via `dotenvy` inside `shared_pool()`.
- One `OnceCell<Option<PgPool>>` for the process — do not open a new pool per test.
- Prefer `fetch_*` helpers that take `&PgPool` (testable) over only `use_context` paths.

## What must gain tests next (publishing phase)

When solo-author write APIs land (ADR-011):

- Auth gate (reject unauthenticated mutate) — **login shell + Argon2/session unit tests landed**
- Create / update / publish / unpublish post
- Slug uniqueness
- Markdown XSS policy (only our renderer; no raw HTML paste without sanitize decision)
- Integration: insert → list → get by slug → delete (transaction/rollback or dedicated test DB)

## Rate limiting

App-level (not only Caddy):

- Global per-IP limiter on almost all routes (`RATE_LIMIT_PUBLIC_PER_MIN`)
- Stricter limiter on login (`RATE_LIMIT_AUTH_PER_MIN`); admin bootstrap is CLI-only
- Trust `X-Forwarded-For` / `X-Real-IP` when behind Caddy

## Agent rules

- New server/domain logic → add unit tests in the same PR/change when feasible
- New SQL / query paths → add or extend soft-gated DB tests
- Do not commit secrets; tests read `.env` locally like the app
