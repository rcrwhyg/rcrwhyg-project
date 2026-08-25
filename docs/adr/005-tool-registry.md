# ADR-005: Static tool registry

## Status

Accepted

## Context

The site will host many small tools over time. Putting every tool into `app.rs` will not scale.

## Decision

- Register tools in `src/tools/registry.rs` as `ToolMeta` entries
- List them on `/tools`
- Each tool still needs an explicit Leptos `Route` until dynamic routing is introduced
- Keep tool UIs under `pages` or future `tools/<id>/` modules

## Consequences

- Adding a tool = registry row + route + page component
- Project skill `leptos-content-and-tools` documents this checklist
