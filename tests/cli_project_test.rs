// tests/integration_test.rs
// Integration tests to verify all functional requirements

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test project initialization
#[test]
fn test_init_creates_project_structure() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    let project_path = temp.path().join("testproj");
    
    // Verify all required files/directories exist
    assert!(project_path.exists(), "Project directory should exist");
    assert!(project_path.join("Cargo.toml").exists(), "Workspace Cargo.toml should exist");
    assert!(project_path.join("core-lib").exists(), "core-lib should exist");
    assert!(project_path.join("core-lib/src/lib.rs").exists(), "core-lib/src/lib.rs should exist");
    assert!(project_path.join("tests").exists(), "tests directory should exist");
    assert!(project_path.join(".cargo/config.toml").exists(), ".cargo/config.toml should exist");
    assert!(project_path.join("glue.toml").exists(), "glue.toml should exist");
    assert!(project_path.join("README.md").exists(), "README.md should exist");
}

/// Test that core-lib is no_std by default
#[test]
fn test_core_lib_is_no_std() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    let lib_content = fs::read_to_string(
        temp.path().join("testproj/core-lib/src/lib.rs")
    ).unwrap();
    
    assert!(
        lib_content.contains("#![cfg_attr(not(feature = \"std\"), no_std)]"),
        "core-lib should be no_std compatible"
    );
}

/// Test adding a platform
#[test]
fn test_add_platform() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    // First init project
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Then add platform
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("add-platform")
        .arg("stm32")
        .arg("--target")
        .arg("thumbv7em-none-eabi")
        .assert()
        .success();
    
    let project_path = temp.path().join("testproj");
    
    // Verify platform files created
    assert!(project_path.join("hal-stm32").exists(), "HAL crate should exist");
    assert!(project_path.join("app-stm32").exists(), "App crate should exist");
    assert!(project_path.join("app-stm32/memory.x").exists(), "Memory.x should exist for embedded target");
    
    // Verify glue.toml updated
    let glue_content = fs::read_to_string(project_path.join("glue.toml")).unwrap();
    assert!(glue_content.contains("stm32"), "glue.toml should contain platform");
    assert!(glue_content.contains("thumbv7em-none-eabi"), "glue.toml should contain target");
}

/// Test listing platforms
#[test]
fn test_list_platforms() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Add a platform
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("add-platform")
        .arg("esp32")
        .arg("--target")
        .arg("xtensa-esp32-none-elf")
        .assert()
        .success();
    
    // List platforms
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("list-platforms")
        .assert()
        .success()
        .stdout(predicate::str::contains("esp32"))
        .stdout(predicate::str::contains("xtensa-esp32-none-elf"));
}

/// Test build command
#[test]
fn test_build_host() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Build for host (would actually run cargo build)
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("build")
        .assert()
        .success();
}

/// Test that generated workspace can run tests
#[test]
fn test_generated_tests_work() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // The test command would run cargo test
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("test")
        .assert()
        .success();
}

/// Test glue config validation
#[test]
fn test_glue_validate() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Add platform
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("add-platform")
        .arg("nrf52")
        .arg("--target")
        .arg("thumbv7em-none-eabihf")
        .assert()
        .success();
    
    // Validate glue config
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("glue")
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validating platform 'nrf52'"));
}

/// Test that workspace Cargo.toml is properly updated
#[test]
fn test_workspace_members_updated() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Add multiple platforms
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("add-platform")
        .arg("stm32")
        .arg("--target")
        .arg("thumbv7em-none-eabi")
        .assert()
        .success();
    
    let workspace_content = fs::read_to_string(
        temp.path().join("testproj/Cargo.toml")
    ).unwrap();
    
    assert!(workspace_content.contains("hal-stm32"), "Workspace should include HAL crate");
    assert!(workspace_content.contains("app-stm32"), "Workspace should include app crate");
}

/// Test that proper panic handler is added for embedded targets
#[test]
fn test_embedded_target_has_panic_handler() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Add embedded platform
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("add-platform")
        .arg("cortex")
        .arg("--target")
        .arg("thumbv7m-none-eabi")
        .assert()
        .success();
    
    let app_cargo = fs::read_to_string(
        temp.path().join("testproj/app-cortex/Cargo.toml")
    ).unwrap();
    
    assert!(app_cargo.contains("panic-halt"), "Embedded app should have panic handler");
    assert!(app_cargo.contains("cortex-m-rt"), "Embedded app should have runtime");
    
    let main_content = fs::read_to_string(
        temp.path().join("testproj/app-cortex/src/main.rs")
    ).unwrap();
    
    assert!(main_content.contains("#![no_std]"), "Embedded main should be no_std");
    assert!(main_content.contains("#![no_main]"), "Embedded main should be no_main");
}

/// Test that desktop targets don't have embedded-specific setup
#[test]
fn test_desktop_target_standard_setup() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Add desktop platform
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("add-platform")
        .arg("linux")
        .arg("--target")
        .arg("x86_64-unknown-linux-gnu")
        .assert()
        .success();
    
    let app_cargo = fs::read_to_string(
        temp.path().join("testproj/app-linux/Cargo.toml")
    ).unwrap();
    
    assert!(!app_cargo.contains("panic-halt"), "Desktop app shouldn't have panic-halt");
    assert!(!app_cargo.contains("cortex-m-rt"), "Desktop app shouldn't have cortex-m-rt");
    
    let main_content = fs::read_to_string(
        temp.path().join("testproj/app-linux/src/main.rs")
    ).unwrap();
    
    assert!(!main_content.contains("#![no_std]"), "Desktop main should have std");
    assert!(main_content.contains("fn main()"), "Desktop should have standard main");
}

/// Test glue configuration management commands
#[test]
fn test_glue_commands() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    
    // First init project
    cmd.current_dir(&temp)
        .arg("init")
        .arg("testproj")
        .assert()
        .success();
    
    // Test glue list on empty project
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("glue")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No platforms configured"));
    
    // Test glue validate on empty project
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("glue")
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validation complete"));
    
    // Test glue init with invalid URL (should fail gracefully)
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("testproj"))
        .arg("glue")
        .arg("init")
        .arg("testplatform")
        .arg("https://github.com/nonexistent/repo")
        .assert()
        .failure(); // Should fail but not crash
}

/// Functional requirement test: Complete workflow
#[test]
fn test_complete_workflow_success_criterion() {
    let temp = TempDir::new().unwrap();
    
    // Step 1: Run tool init myproj
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(&temp)
        .arg("init")
        .arg("myproj")
        .assert()
        .success();
    
    // Step 2: Run tool test (see working unit tests on host)
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("myproj"))
        .arg("test")
        .assert()
        .success();
    
    // Step 3: Run tool add-platform stm32 --target thumbv7em-none-eabi
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("myproj"))
        .arg("add-platform")
        .arg("stm32")
        .arg("--target")
        .arg("thumbv7em-none-eabi")
        .assert()
        .success();
    
    // Step 4: Run tool build --target stm32 (without modifying configs manually)
    let mut cmd = Command::cargo_bin("multi-target-rs").unwrap();
    cmd.current_dir(temp.path().join("myproj"))
        .arg("build")
        .arg("--target")
        .arg("stm32")
        .assert()
        .success();
    
    // Verify all artifacts exist as expected
    let project = temp.path().join("myproj");
    assert!(project.join("core-lib").exists());
    assert!(project.join("tests").exists());
    assert!(project.join("hal-stm32").exists());
    assert!(project.join("app-stm32").exists());
    assert!(project.join("glue.toml").exists());
    
    // Verify glue.toml has the platform
    let glue = fs::read_to_string(project.join("glue.toml")).unwrap();
    assert!(glue.contains("stm32"));
    assert!(glue.contains("thumbv7em-none-eabi"));
    
    println!("✅ Success criterion met: Complete workflow executed without manual config editing");
}
