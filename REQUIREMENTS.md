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
- [x] **Glue Configuration Management**
  - [x] Command: `tool glue init <platform> <github-url>` - Initialize from package URL
  - [x] Command: `tool glue list` - List configured platforms with analysis
  - [x] Command: `tool glue remove <platform>` - Remove platform configuration
  - [x] Command: `tool glue validate` - Validate configurations and HAL compatibility
  - [x] Package inspection from GitHub URLs (e.g., https://github.com/stm32-rs/stm32f4xx-hal)
  - [x] Automatic trait discovery and analysis using Rust AST parsing
  - [x] Native trait compatibility analysis and warnings
  - [x] Mock trait identification for testing compatibility
- [ ] **Custom Templates**
  - [ ] Allow adding project templates for specific platforms (e.g. STM32, ESP32)
- [ ] **CI/CD Integration**
  - [ ] Generate GitHub Actions workflow for multi-target builds
- [ ] **Future-Proofing**
  - [ ] Hooks for logging frameworks (e.g., `defmt`)
  - [ ] Extendable config for non-HAL abstractions

### 7.1 Package Inspection Features
- [x] **GitHub URL Analysis**: Parse repository URLs and fetch source code
- [x] **Trait Discovery**: Identify implemented and provided traits using `syn` parser
- [x] **Native Compatibility**: Analyze which traits can be mocked on native platforms
- [x] **Dependency Analysis**: Extract HAL dependencies and requirements
- [x] **Target Inference**: Smart target triple detection based on repository patterns
- [x] **Configuration Storage**: Persist analysis results in `glue.toml` with detailed metadata

### 7.2 Example User Story
```bash
# User provides GitHub URL
tool glue init stm32f4 https://github.com/stm32-rs/stm32f4xx-hal

# Tool inspects package and displays analysis:
# 📊 Package Analysis Results:
#   Source: https://github.com/stm32-rs/stm32f4xx-hal
#   Version: 0.22.1
#   📦 Found 15 traits:
#     ✅ OutputPin (module: lib) - Native mockable
#     ✅ InputPin (module: lib) - Native mockable  
#     ⚠️  CustomTrait (module: hal) - May not be available for native testing
#   🧪 Native mockable traits: OutputPin, InputPin, Spi, I2c
#   ⚠️  Warnings:
#     - Trait 'CustomTrait' may not be available for native testing

# Configuration saved with detailed trait analysis for native testing
```

---

✅ **Success Criterion**:  
A developer can:
1. ✅ Run `tool init myproj`
2. ✅ Run `tool test` and see working unit tests on host
3. ✅ Run `tool add-platform stm32 --target thumbv7em-none-eabi`
4. ✅ Run `tool build --target stm32` without modifying configs manually
5. ✅ Add validated glue configs to support new boards incrementally

