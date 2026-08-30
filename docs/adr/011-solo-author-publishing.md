# ADR-011: Solo-author admin auth + publishing path

## Status

Accepted — admin session + CLI bootstrap; **publishing via `articles/` git workflow** (no DB post CRUD)

## Context

A personal site needs a secure owner path. Auth patterns from prior Axum work (Argon2id); transport is **Postgres sessions + HttpOnly cookies**.

Publishing is **file-based**: `articles/<合集>/NN-slug.md` (or root-level essays) → git → CD → `/articles`. Browser admin is for ops/login shell, not WYSIWYG editing.

## Decisions

| Topic | Choice |
|-------|--------|
| Sessions | `admin_sessions` table; cookie holds raw token; DB stores SHA-256 hex |
| Cookie | `HttpOnly`, `SameSite=Strict`, `Path=/`; `Secure` when `COOKIE_SECURE=true` or `LEPTOS_ENV=PROD` |
| Admin bootstrap | **CLI only**: `create-admin` on server shell |
| Entry | `/admin` + `/admin/login`; header shows **后台** / **退出** when logged in |
| Password | Argon2id; min length 12 |
| Publishing | Edit `articles/`, run CI/CD — **not** `/admin/posts` |
| Collections | Subdir + `_meta.json` (`title`, optional `placeholder`); slug from filename |
| Rate limit | Public site limiter + stricter auth limiter on login |

## Bootstrap (ops)

```bash
cargo run --features ssr --bin create-admin -- you@example.com 'at-least-12-chars'
```

Fails if an admin already exists.

## Schema

See `sql/auth.sql`. `sql/posts.sql` is legacy — do not use for new content.
