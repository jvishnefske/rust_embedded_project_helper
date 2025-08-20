# Multi-Target Rust Project Tool

A command-line utility for creating and managing **cross-platform, native-testable Rust projects** with clean separation of hardware-agnostic logic, hardware abstraction layers, and platform-specific binaries.  

This tool solves the problem of scaling embedded Rust projects from a simple single-crate demo to a **production-ready, multi-crate Cargo workspace** — enabling:
- ⚡ **Native host-based testing** (fast unit tests with mocks)
- 🔌 **Hardware abstraction via `embedded-hal`**
- 🛠 **Cross-compilation for multiple targets** (e.g. STM32, ESP32, desktop)
- 🚀 **Minimal dependencies until needed** (no HALs required unless added)

---

## ✨ Features
- `init` → Scaffold a new multi-crate workspace (`core-lib`, tests, configs).
- `add-platform` → Add a target platform with HAL + binary glue crate.
- `build` → Build for host or specific target (via Cargo or Cross).
- `test` → Run fast, native unit tests using mocks.
- `glue configs` → Manage validated platform configurations without editing text files manually.
- CI/CD friendly (optional GitHub Actions workflow scaffold).

---

## 📦 Installation
```bash
cargo install multi-target-rs
````

(Replace `multi-target-rs` with the actual crate name once published.)

---

## 🚀 Quick Start

### 1. Initialize a new project

```bash
tool init myproj
cd myproj
```

Creates a Cargo workspace with:

* `core-lib/` (hardware-agnostic logic, `#[no_std]`)
* `tests/` (native host-based tests with mocks)
* `.cargo/config.toml` (minimal defaults)

---

### 2. Run unit tests on host

```bash
tool test
```

Runs template unit tests using `embedded-hal-mock`.
✅ Works out of the box — no hardware required.

---

### 3. Add a hardware platform

```bash
tool add-platform stm32 --target thumbv7em-none-eabi
```

Scaffolds:

* `hal-stm32/` (HAL wrapper crate)
* `app-stm32/` (platform-specific binary)
* Updates workspace `Cargo.toml`

---

### 4. Build for specific targets

```bash
tool build --target stm32
```

* Uses `cross` (if available) or `cargo build`
* Ensures correct toolchain/linker setup from `glue.toml`

---

### 5. Run tests on target hardware

```bash
tool test --target stm32
```

Uses [`probe-rs`](https://github.com/probe-rs/probe-rs) + [`embedded-test`](https://github.com/probe-rs/embedded-test) for flashing & test execution on device.

---
## 🧩 Commands

```bash
tool init <project-name>        # Initialize new project
tool add-platform <name> --target <triple>
tool list-platforms             # Show added platforms
tool build [--target <name>]    # Build for host or target
tool test [--target <name>]     # Run tests on host or target
```

---

## 📖 Example Workflow

```bash
# Create new project
tool init myproj
cd myproj

# Verify native tests work
tool test

# Add STM32 platform
tool add-platform stm32 --target thumbv7em-none-eabi

# Build for STM32
tool build --target stm32

# Run on hardware
tool test --target stm32
```

---

## 🛠 Extensibility

* Add custom templates for new boards.
* Generate CI/CD pipelines (`.github/workflows`).
* Integrates seamlessly with `defmt`, `probe-rs`, and `cross`.

---

## 🔮 Roadmap

* [ ] Expand template library (ESP32, nRF, RP2040, RISC-V).
* [ ] Add config wizard for common toolchains.
* [ ] Support mixed-language crates (Rust + C/C++).
* [ ] Automatic integration with `cargo-generate`.

---

## License

MIT or BSD


