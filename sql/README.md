# SQL schemas

| File | Role |
|------|------|
| [`posts.sql`](posts.sql) | Canonical site-owned posts (+ tags) |
| [`seed_posts.sql`](seed_posts.sql) | Optional seed (`plain-start` essay) |
| [`auth.sql`](auth.sql) | Solo admin + session tables |
| [`articles.sql`](articles.sql) | **Legacy** — do not extend |

## Apply (local)

```bash
psql "$DATABASE_URL" -f sql/posts.sql -f sql/seed_posts.sql -f sql/auth.sql
```

Then create the solo admin from the server shell (not the browser):

```bash
cargo run --features ssr --bin create-admin -- you@example.com 'at-least-12-chars'
```

Open `/admin/login` to sign in.
