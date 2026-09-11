# Coding Platform Agent Guide

## Scope and source of truth

These instructions apply to the whole repository. A nested `AGENTS.md` adds or
overrides instructions for files in its directory.

Read the relevant files in `docs/` before changing product behavior,
architecture, persistence, API contracts, or execution security.

Document priority:

1. `docs/PRD.md` is the product source of truth.
2. `docs/USER-FLOW.md` and `docs/INFORMATION-ARCHITECTURE.md` define behavior and UX.
3. `docs/API-CONTRACT.md` defines the public HTTP contract.
4. `docs/DATA-MODEL.md` and `docs/ERD.md` define persistence.
5. `docs/TECHNICAL-DESIGN.md`, `docs/EXECUTION-ARCHITECTURE.md`,
   `docs/INFRASTRUCTURE-DESIGN.md`, and `docs/REPOSITORY-STRUCTURE.md` define
   implementation boundaries.

If a lower-priority document conflicts with the PRD, follow the PRD and update
the derived document in the same change. Do not silently choose a conflicting
interpretation.

In technical names and routes, `Course` is the delivery representation of the
PRD's `Learning Path`. Product copy should prefer `Learning Path` when that is
clearer for learners.

## Repository layout

- `web/`: Astro frontend. Follow `web/AGENTS.md` for frontend-specific work.
- `api/`: Rust backend and composition root.
- `api/src/<module>/`: bounded contexts and their layers.
- `runner/`: versioned runner images and execution protocol.
- `migrations/`: versioned PostgreSQL migrations.
- `tests/`: cross-module, API, integration, and fixture-based tests.
- `docs/`: product and architecture decisions.

Keep `api/src/main.rs` thin. It may load configuration, initialize
infrastructure, compose modules, register routes, and start the server; it must
not contain business rules.

## MVP product rules

The PRD includes these MVP areas:

- Learn: Learning Path/Course, Section, Lesson, Exercise, and Test Case.
- Practice: standalone Problems with difficulty, topics, selectable languages,
  test cases, and submissions.
- Playground: free code execution with a language selector; Run does not award
  XP or change learning completion.
- Identity: registration, login, logout, guest access, and persistent progress
  for authenticated users.
- Gamification: XP and derived Level. Badge is a should-have, not a reason to
  introduce a generic rules engine.
- Content management: minimum Admin workflows for creating, editing, testing,
  publishing, and unpublishing content.

Do not move a PRD MVP capability out of scope merely because its API or table
design is incomplete. Complete the relevant contract first.

Core invariants:

- An Exercise has exactly one configured programming language. The client
  cannot override it during submission.
- A Practice Problem may support multiple languages selected by the user.
- Run executes code but never marks content complete and never awards XP.
- Submit evaluates all required test cases.
- Exercise completion is binary: all required tests pass or the submission
  fails. There is no partial score or partial XP.
- The first successful Exercise completion awards 100 XP. Later successful
  submissions for the same user and Exercise award zero additional XP.
- A Lesson is complete only when all its published required Exercises are
  complete.
- Published Lessons are accessible without hard locking. Recommend the first
  incomplete Lesson, but allow out-of-order learning.
- Guest access may expose selected learning, practice, or playground content.
  Authentication is required for persistent submissions, progress, XP, Level,
  and other personal state.
- Lessons are reusable across Learning Paths through ordered Section
  membership; do not duplicate lesson content to simulate reuse.

## Data and API rules

- PostgreSQL is the application source of truth and uses schema/module
  ownership.
- Store facts and derive state. Exercise completion, Lesson completion, Course
  progress, recommended Lesson, total XP, and Level are derived for the MVP.
- Derive Exercise completion from successful submission history; do not add
  progress tables without an explicit documented decision changing this model.
- Store XP in an append-only transaction ledger and protect one-time awards with
  a database unique constraint and transaction.
- Use UUID primary keys and UTC `TIMESTAMPTZ` values consistently.
- Use foreign keys inside a bounded context. Cross-context references are IDs
  accessed through public application contracts, not arbitrary table access.
- Prefer publish/unpublish or soft deletion for learning content referenced by
  submission history.
- Public HTTP endpoints use `/api/v1` and JSON envelopes from
  `docs/API-CONTRACT.md`.
- Error codes are stable frontend contracts; human-readable messages may evolve.
- Never expose hidden test cases, internal module structure, or another user's
  submissions.
- Identity comes from the authentication context, never a client-provided
  `user_id`.

## Architecture boundaries

Organize backend code by capability, then by layer:

```text
<module>/
├── domain/
├── application/
├── infrastructure/
└── presentation/
```

- Domain code contains business rules and must not depend on Axum, SQLx,
  container-runtime clients, or HTTP types.
- Application code orchestrates use cases, authorization, transactions,
  repositories, runner ports, and cross-module contracts.
- Infrastructure implements persistence (concrete SQLx repositories) and external adapters.
  Avoid speculative trait abstractions in domain for repositories when there is only a single PostgreSQL persistence implementation.
- Presentation translates HTTP requests and responses; it contains no business
  rules.
- Modules must not import another module's infrastructure, presentation, or
  private domain internals.
- Keep shared code limited to genuinely cross-cutting technical concerns. Never
  place domain logic in a generic `shared` module.
- Use direct in-process application contracts for MVP. Do not add
  microservices, queues, brokers, Redis, Kubernetes, or event infrastructure
  without a demonstrated requirement and a documented decision.

## Untrusted code execution

Treat every user program as hostile.

- Never execute user code directly in the API process or as an unrestricted
  host process.
- Execute in an ephemeral, non-privileged container through a `Runner` port.
- Local development uses Podman. Keep production runtime details behind the
  `ContainerRunner` abstraction.
- Enforce hard timeout, CPU, memory, process-count, output-size, request-size,
  and source-size limits outside the user process.
- Disable network access and prevent host filesystem, host namespace, runtime
  socket, database, and application-secret access.
- Use versioned immutable runner images; do not depend on `latest` in production.
- Always clean up containers on success, failure, timeout, runner error, and
  application shutdown.
- Do not log submitted source code or inject application credentials into the
  runner.
- Distinguish user-code failures from infrastructure failures. Do not report a
  runner startup failure as a failed solution.
- Apply bounded concurrency and submission rate limiting before creating a
  container.

## Development and verification

Run commands from the component directory unless stated otherwise.

Frontend (`web/`):

```sh
ASTRO_TELEMETRY_DISABLED=1 npm run build
```

Run `astro check` only after the repository adds and configures
`@astrojs/check`; it is not part of the current frontend scaffold.

Use the background development-server workflow required by `web/AGENTS.md`.

Backend (`api/`):

```sh
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Add tests in proportion to the change:

- Unit tests for domain invariants, validation, XP, completion, and Level logic.
- Integration tests for repositories, transactions, and module contracts.
- API tests for authentication, authorization, validation, envelopes, and error
  codes.
- End-to-end tests for the core learn-to-submit-to-XP flow.
- Runner tests for normal execution, infinite loops, memory pressure, process
  abuse, oversized output, compile/runtime errors, network isolation, and cleanup.

Do not claim a check passed unless it was run. If a command is unavailable
because the corresponding component has not been implemented, report it as not
applicable rather than fabricating a result.

## Change discipline

- Keep changes small and aligned with the current MVP; avoid speculative
  abstractions and empty architectural scaffolding.
- Preserve existing public contracts unless the task explicitly authorizes a
  breaking change.
- Update PRD-derived documentation whenever behavior, API shape, schema, routes,
  module ownership, or runner security changes.
- Add migrations as versioned repository files; never edit production schema by
  hand as the only record of a change.
- Never commit secrets or real `.env` files. Keep `.env.example` to placeholders.
- Before finishing, review the diff, run relevant checks, and summarize any
  intentionally deferred PRD contract details.
