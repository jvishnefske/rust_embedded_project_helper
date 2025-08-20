# Functional Requirements Checklist  
**Command-Line Tool for Multi-Target, Native-Testable Rust Projects**

## 1. Project Initialization
- [x] **New Project Creation**
  - [x] Command: `tool init <project-name>`
  - [x] Create a Cargo workspace with:
    - [x] Core library crate (`core-lib`) – `#[no_std]`, hardware-agnostic
    - [x] Test harness scaffolding (host-based)
    - [x] Minimal dependencies (no HALs until added)
- [x] Generate `.cargo/config.toml` with sane defaults
- [x] Include README with usage instructions

## 2. Platform Management
- [x] **Add Platform**
  - [x] Command: `tool add-platform <platform-name> --target <triple>`
  - [x] Create hardware-specific crate for HAL integration
  - [x] Scaffold application binary crate (`app-<platform>`)
  - [x] Add platform to workspace members in `Cargo.toml`
  - [x] Add glue config linking HAL + core-lib
- [x] **List Platforms**
  - [x] Command: `tool list-platforms`
  - [x] Show registered platforms and their target triples

## 3. Build System Integration
- [x] **Default Builds**
  - [x] Command: `tool build` builds core-lib + host tests
- [x] **Targeted Builds**
  - [x] Command: `tool build --target <platform>`
  - [x] Use `cross` or `cargo` depending on target
  - [x] Verify correct linker/toolchain setup via `glue.toml`
- [x] **Minimal Dependency Resolution**
  - [x] Ensure core-lib builds without embedded HALs unless target is added
  - [x] Enforce dependency scoping per crate

## 4. Testing
- [x] **Native Unit Tests**
  - [x] Command: `tool test`
  - [x] Run core-lib tests on host with `embedded-hal-mock`
- [x] **Template Tests**
  - [x] Provide scaffolded example tests (mocks, stubs)
  - [x] Ensure test runs without editing Cargo config manually
- [ ] **On-Target Tests**
  - [ ] Command: `tool test --target <platform>`
  - [ ] Integrate with `probe-rs` + `embedded-test` where available

## 5. Configuration Management
- [x] **Glue Configs**
  - [x] Support `glue.toml` for mapping HAL crates to targets
  - [x] Validate configs before inclusion
  - [ ] Allow multiple validated configs per platform
- [x] **Workspace Consistency**
  - [x] Ensure workspace `Cargo.lock` stays synchronized across platforms
  - [x] Auto-update dependencies when adding/removing targets

## 6. User Experience
- [x] **Scaffold Without Manual Editing**
  - [x] All generated crates and configs ready-to-build out-of-box
  - [x] No need for user to touch `Cargo.toml` or `.cargo/config.toml`
- [x] **Clear CLI Feedback**
  - [x] Inform user of added platforms, targets, and test runs
  - [x] Provide hints if dependencies/toolchains are missing

## 7. Extensibility
- [ ] **Custom Templates**
  - [ ] Allow adding project templates for specific platforms (e.g. STM32, ESP32)
- [ ] **CI/CD Integration**
  - [ ] Generate GitHub Actions workflow for multi-target builds
- [ ] **Future-Proofing**
  - [ ] Hooks for logging frameworks (e.g., `defmt`)
  - [ ] Extendable config for non-HAL abstractions

---

✅ **Success Criterion**:  
A developer can:
1. ✅ Run `tool init myproj`
2. ✅ Run `tool test` and see working unit tests on host
3. ✅ Run `tool add-platform stm32 --target thumbv7em-none-eabi`
4. ✅ Run `tool build --target stm32` without modifying configs manually
5. ✅ Add validated glue configs to support new boards incrementally

