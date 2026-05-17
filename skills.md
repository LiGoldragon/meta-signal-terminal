# skills — owner-signal-persona-terminal

Read this before editing the owner-only terminal contract.

## Required context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `signal-persona-terminal/ARCHITECTURE.md`
- `persona-terminal/ARCHITECTURE.md`

## Boundary

This crate owns the privileged OwnerSignal vocabulary for
`persona-terminal` session lifecycle. It contains no daemon code, no
actors, no sockets, and no storage code.

The ordinary `signal-persona-terminal` crate owns the normal terminal
communication surface: input, resize, capture, prompt patterns, input
gates, worker lifecycle, and read-only session lookup. This crate owns
starting and retiring terminal sessions.

## Invariants

- `CreateSession` and `RetireSession` live here, not in the ordinary
  terminal contract.
- Every request variant declares a Signal root verb through
  `signal_channel!`.
- Shared terminal nouns such as `TerminalName` and `TerminalExitStatus`
  are imported from `signal-persona-terminal`; do not duplicate them.
- Runtime interpretation stays in `persona-terminal`.
