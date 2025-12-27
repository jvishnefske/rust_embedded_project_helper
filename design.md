# Design Document: Multi-Target Rust Project Tool

## Overview

A command-line utility for creating and managing cross-platform, native-testable Rust embedded projects with clean separation of hardware-agnostic logic, hardware abstraction layers, and platform-specific binaries.

## MVP Functional Requirements

### 1. Project Initialization

- [x] FR-1.1: `init <project-name>` creates a Cargo workspace with core-lib, tests, and configuration
- [x] FR-1.2: Core library crate is `#[no_std]` compatible and hardware-agnostic
- [x] FR-1.3: Test harness is scaffolded for host-based testing with mocks
- [x] FR-1.4: `.cargo/config.toml` is generated with sane defaults
- [x] FR-1.5: `glue.toml` is created for platform configuration

### 2. Platform Management

- [x] FR-2.1: `add-platform <name> --target <triple>` creates HAL wrapper and app crates
- [x] FR-2.2: Workspace `Cargo.toml` is updated with new members
- [x] FR-2.3: `glue.toml` is updated with platform configuration
- [x] FR-2.4: `list-platforms` displays registered platforms and targets
- [x] FR-2.5: Embedded targets include `memory.x` and panic handler setup

### 3. Build System

- [x] FR-3.1: `build` without target builds core-lib and tests for host
- [x] FR-3.2: `build --target <platform>` builds for specified platform
- [x] FR-3.3: Automatic tool selection between cargo and cross
- [x] FR-3.4: Build preferences are persisted in `glue.toml`

### 4. Testing

- [x] FR-4.1: `test` runs native unit tests with `embedded-hal-mock`
- [x] FR-4.2: Template tests are provided and work out of box
- [ ] FR-4.3: `test --target <platform>` runs on-target tests via probe-rs

### 5. Glue Configuration

- [x] FR-5.1: `glue init <platform> <url>` fetches and analyzes HAL from GitHub
- [x] FR-5.2: `glue list` shows platforms with trait analysis
- [x] FR-5.3: `glue validate` checks configuration consistency
- [x] FR-5.4: `glue remove <platform>` removes platform configuration
- [x] FR-5.5: Automatic trait discovery using Rust AST parsing
- [x] FR-5.6: Native trait compatibility analysis with warnings

### 6. User Experience

- [x] FR-6.1: All generated crates build without manual editing
- [x] FR-6.2: Clear CLI feedback with progress indicators
- [x] FR-6.3: Helpful error messages with suggested fixes

## Architectural Principles

### Immutable-by-Default

All configuration structures (`GlueConfig`, `Platform`, `HalInfo`) are modeled as immutable data. State transitions return new instances.

### Functional Core, Imperative Shell

- **Functional Core**: Configuration parsing, trait analysis, template generation
- **Imperative Shell**: File I/O, network requests, process execution

### Linear Ownership

The tool maintains a simple ownership structure:
- `MultiTargetTool` owns the project root path
- `PackageInspector` owns an HTTP client
- No shared mutable state or reference cycles

### Type-Driven Design

- `Commands` and `GlueCommands` enums model all valid CLI states
- `BuildTool` enum restricts tool selection to known values
- `Platform` struct enforces required fields at compile time

### Explicit Error Handling

All fallible operations return `Result<T, E>`:
- `anyhow::Error` for application-level errors
- `Box<dyn std::error::Error>` for CLI operations
- Errors propagate with context to provide actionable messages

## Success Criteria

A developer can execute the following workflow without manual configuration:

1. `multi-target-rs init myproj`
2. `multi-target-rs test` (native tests pass)
3. `multi-target-rs add-platform stm32 --target thumbv7em-none-eabi`
4. `multi-target-rs build --target stm32`
5. Add validated glue configs for new boards incrementally

## Traceability

| Requirement | Test Coverage |
|-------------|---------------|
| FR-1.1 | `test_init_creates_project_structure` |
| FR-1.2 | `test_core_lib_is_no_std` |
| FR-1.3 | `test_generated_tests_work` |
| FR-2.1 | `test_add_platform` |
| FR-2.2 | `test_workspace_members_updated` |
| FR-2.4 | `test_list_platforms` |
| FR-2.5 | `test_embedded_target_has_panic_handler` |
| FR-3.1 | `test_build_host` |
| FR-3.2 | `test_complete_workflow_success_criterion` |
| FR-4.1 | `test_generated_tests_work` |
| FR-5.3 | `test_glue_validate` |
| FR-6.1 | `test_complete_workflow_success_criterion` |
| FR-6.2 | `test_desktop_target_standard_setup` |
