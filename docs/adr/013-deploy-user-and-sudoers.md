# ADR-013: Dedicated `rcrwhyg` user with narrow sudoers

## Status

Accepted (2026-08-25).

## Context

The CD pipeline needs to (a) write files under `/opt/rcrwhyg/` and
(b) run `systemctl {start,stop,restart,status} rcrwhyg.service` plus
`journalctl -u rcrwhyg.service *`. Three options for the SSH user:

1. **`root`** — simplest, but the CD pipeline can do anything on the box
   (modify other services, read `/etc/shadow`, etc.). If the deploy key
   leaks, full VPS compromise.
2. **A systemd user instance (`systemctl --user`)** — purest, no sudo at
   all, but requires `loginctl enable-linger rcrwhyg`, an
   `XDG_RUNTIME_DIR`, and Caddy must be configured to trust a
   user-scope socket (or talk to the port directly, which it already
   does). The extra moving parts don't pay off for a single service.
3. **A dedicated `rcrwhyg` system user with a `sudoers.d/rcrwhyg`
   drop-in** allowing only the specific commands the CD needs.**

We chose (3).

## Decision

- `rcrwhyg` is a `--system` user (no password, no expiry) with home
  `/opt/rcrwhyg` and shell `/bin/bash` (so SCP works without extra config).
- `/etc/sudoers.d/rcrwhyg` lists exactly five commands with
  `NOPASSWD`. Anything else is denied.
- Mode `0440` on the drop-in (required by `sudo`).
- `.env` is `root:rcrwhyg 0640` so the service can read secrets but
  neither the deploy key nor any compromise of the process can edit it.

## Trade-offs accepted

- The deploy key has narrow but real power: it can restart the service
  and read the service's journal. If the key leaks, the attacker has
  ~5 minutes to do damage before the user notices and rotates the key
  in GitHub Secrets.
- The user must remember to keep the sudoers drop-in in sync if the CD
  workflow ever needs new commands.

## Consequences

- The systemd unit runs `User=rcrwhyg` / `Group=rcrwhyg`.
- Hardening in the unit (`NoNewPrivileges`, `ProtectSystem=strict`,
  `PrivateTmp`, `ReadWritePaths=/opt/rcrwhyg`) is defense-in-depth, not
  the primary isolation mechanism.
- If the user later moves to multiple services, a per-service
  `sudoers.d/<service>` drop-in pattern scales without change.
