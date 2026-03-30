# Laser Driver Pipeline (inspired by LibLaserCut)

This document defines a reusable execution pipeline for machine-agnostic laser jobs.

## Goals

- Separate **job model** from **machine/controller protocol**.
- Keep one validation entry point before sending any job.
- Allow driver-specific preparation while preserving the current app flow.
- Enable deterministic testing for exported command streams.

## New modules

- `src/laser/job.rs`
  - `LaserJob`: executable job payload + metadata.
  - Tracks start offset and optional rotary parameters.
  - Provides helper methods for emptiness and workspace bounds.

- `src/laser/driver.rs`
  - `LaserDriver` trait with:
    - `validate_job(...)`
    - `prepare_program(...)`
    - `model_name()`
  - `create_driver(ControllerKind)` factory.
  - Initial concrete drivers:
    - `GrblGCodeDriver`
    - `LineProtocolGCodeDriver` (Marlin/Ruida/Trocen)

- `src/laser/pipeline.rs`
  - `prepare_program(controller_kind, machine, job)`
  - Returns `PreparedProgram { driver_name, lines, validation_issues }`.

## Flow

1. Build `LaserJob` from current program lines.
2. Select driver from `MachineProfile.controller_kind`.
3. Run driver validation + workspace sanity checks.
4. Build prepared lines from the selected driver.
5. Send prepared lines through existing serial sender.

## Why this maps well to LibLaserCut ideas

- Matches the `LaserCutter` pattern: one abstract driver contract.
- Keeps preflight checks centralized (`checkJob` equivalent).
- Keeps protocol backends and job representation decoupled.
- Makes output testable independently from hardware transport.

## Suggested next integration step

Wire `prepare_program(...)` into `All4LaserApp` right before runtime sending (currently done in `send_next_program_line`) so all runs pass through the same validation and preparation path.
