# Functional Requirements Checklist  
**Command-Line Tool for Multi-Target, Native-Testable Rust Projects**

## 1. Project Initialization
- [ ] **New Project Creation**
  - [ ] Command: `tool init <project-name>`
  - [ ] Create a Cargo workspace with:
    - [ ] Core library crate (`core-lib`) – `#[no_std]`, hardware-agnostic
    - [ ] Test harness scaffolding (host-based)
    - [ ] Minimal dependencies (no HALs until added)
- [ ] Generate `.cargo/config.toml` with sane defaults
- [ ] Include README with usage instructions

## 2. Platform Management
- [ ] **Add Platform**
  - [ ] Command: `tool add-platform <platform-name> --target <triple>`
  - [ ] Create hardware-specific crate for HAL integration
  - [ ] Scaffold application binary crate (`app-<platform>`)
  - [ ] Add platform to workspace members in `Cargo.toml`
  - [ ] Add glue config linking HAL + core-lib
- [ ] **List Platforms**
  - [ ] Command: `tool list-platforms`
  - [ ] Show registered platforms and their target triples

## 3. Build System Integration
- [ ] **Default Builds**
  - [ ] Command: `tool build` builds core-lib + host tests
- [ ] **Targeted Builds**
  - [ ] Command: `tool build --target <platform>`
  - [ ] Use `cross` or `cargo` depending on target
  - [ ] Verify correct linker/toolchain setup via `glue.toml`
- [ ] **Minimal Dependency Resolution**
  - [ ] Ensure core-lib builds without embedded HALs unless target is added
  - [ ] Enforce dependency scoping per crate

## 4. Testing
- [ ] **Native Unit Tests**
  - [ ] Command: `tool test`
  - [ ] Run core-lib tests on host with `embedded-hal-mock`
- [ ] **Template Tests**
  - [ ] Provide scaffolded example tests (mocks, stubs)
  - [ ] Ensure test runs without editing Cargo config manually
- [ ] **On-Target Tests**
  - [ ] Command: `tool test --target <platform>`
  - [ ] Integrate with `probe-rs` + `embedded-test` where available

## 5. Configuration Management
- [ ] **Glue Configs**
  - [ ] Support `glue.toml` for mapping HAL crates to targets
  - [ ] Validate configs before inclusion
  - [ ] Allow multiple validated configs per platform
- [ ] **Workspace Consistency**
  - [ ] Ensure workspace `Cargo.lock` stays synchronized across platforms
  - [ ] Auto-update dependencies when adding/removing targets

## 6. User Experience
- [ ] **Scaffold Without Manual Editing**
  - [ ] All generated crates and configs ready-to-build out-of-box
  - [ ] No need for user to touch `Cargo.toml` or `.cargo/config.toml`
- [ ] **Clear CLI Feedback**
  - [ ] Inform user of added platforms, targets, and test runs
  - [ ] Provide hints if dependencies/toolchains are missing

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
1. Run `tool init myproj`
2. Run `tool test` and see working unit tests on host
3. Run `tool add-platform stm32 --target thumbv7em-none-eabi`
4. Run `tool build --target stm32` without modifying configs manually
5. Add validated glue configs to support new boards incrementally

