# Multi-Target Rust Project Tool

Scaffold and manage cross-platform, native-testable Rust embedded projects with a single CLI. Write hardware-agnostic logic once, test locally with mocks, and cross-compile to any target without manual Cargo.toml editing.

## Quick Start

```bash
cargo install --path .
multi-target-rs init myproj && cd myproj
multi-target-rs test
multi-target-rs add-platform stm32 --target thumbv7em-none-eabi
```

## Overview

This tool solves the problem of scaling embedded Rust projects from a simple single-crate demo to a production-ready, multi-crate Cargo workspace. It enables:

- Native host-based testing with `embedded-hal-mock`
- Hardware abstraction via `embedded-hal` traits
- Cross-compilation for multiple targets (STM32, ESP32, nRF, desktop)
- Minimal dependencies until needed (no HALs required unless added)
- CI/CD friendly workflow scaffolding

## Commands

| Command | Description |
|---------|-------------|
| `init <project-name>` | Initialize new project with workspace, core-lib, and tests |
| `add-platform <name> --target <triple>` | Add a target platform with HAL + binary crate |
| `list-platforms` | Show registered platforms and their target triples |
| `build [--target <name>]` | Build for host or specific target |
| `test [--target <name>]` | Run tests on host or target hardware |
| `glue init <platform> <url>` | Initialize glue config from GitHub HAL repository |
| `glue list` | List configured platforms with trait analysis |
| `glue validate` | Validate configurations and HAL compatibility |

## Project Structure

After initialization, your project contains:

```
myproj/
  Cargo.toml          # Workspace manifest
  glue.toml           # Platform and HAL configuration
  .cargo/config.toml  # Build defaults
  core-lib/           # Hardware-agnostic business logic (#[no_std])
  tests/              # Host-based unit tests with mocks
  hal-<platform>/     # HAL wrapper crates (added via add-platform)
  app-<platform>/     # Platform-specific binaries (added via add-platform)
```

## Example Workflow

```bash
# Create and test locally
multi-target-rs init myproj
cd myproj
multi-target-rs test              # Run unit tests with mocks

# Add embedded target
multi-target-rs add-platform stm32 --target thumbv7em-none-eabi
multi-target-rs build --target stm32

# Inspect HAL package for trait compatibility
multi-target-rs glue init stm32f4 https://github.com/stm32-rs/stm32f4xx-hal
multi-target-rs glue list
```

## License

MIT OR Apache-2.0
