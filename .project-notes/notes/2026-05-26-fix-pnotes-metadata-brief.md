---
id: 2026-05-26-fix-pnotes-metadata-brief
type: continuity
task: fix-pnotes-metadata-brief
created_at: 2026-05-26
signal: Fixed pnotes CLI bugs related to supersedes filtering, brief limit, raw parser equals support, notes ID derivation, and brief recent notes formatting.
areas:
- project-notes/pnotes-cli
tags:
- rust
tests:
- command: cargo test
  covers:
  - brief supersedes matches logic
  - brief respects limit
  - raw args equals support
  - id filename derivation
  - recent notes printing
---

## task
fix-pnotes-metadata-brief — 2026-05-26

## deviations
None

## traps
None

## dead_ends
None

## validation_delta
As expected

## next_agent_hint
See Handoff
