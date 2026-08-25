# ADR-006: Diversified site areas

## Status

Accepted

## Context

The product is a personal site that will grow beyond blog posts and tools (notes, galleries, experiments, playgrounds, etc.). Hard-coding only “blog + tools” into architecture and agent skills would force rewrites later.

## Decision

- Treat the site as a set of **areas** behind one router and one chrome shell.
- Blog and tools are the first two areas, not the ceiling.
- New areas follow: domain types (if needed) → `pages/` + route → optional registry entry → header nav link.
- Prefer an `areas` registry (planned, similar to `tools/registry.rs`) for discoverability in nav / indexes.

## Consequences

- Skills and docs speak of “site areas”, not “only blog/tools”
- Agents must not refuse new sections for being “out of scope” if they fit the personal-site product
