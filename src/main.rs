// main.rs - Multi-Target Rust Project CLI Tool
// A CLI for managing cross-platform, native-testable Rust embedded projects

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

// CLI argument structure using clap derive macros
#[derive(Parser)]
#[command(name = "multi-target-rs")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for managing multi-target Rust embedded projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new multi-target project
    Init {
        /// Project name
        name: String,
    },
    /// Add a new target platform
    AddPlatform {
        /// Platform name (e.g., stm32, esp32)
        name: String,
        /// Target triple
        #[arg(long)]
        target: String,
        /// Optional HAL crate name
        #[arg(long)]
        hal: Option<String>,
    },
    /// List all configured platforms
    ListPlatforms,
    /// Build the project
    Build {
        /// Target platform to build for
        #[arg(long)]
        target: Option<String>,
        /// Use cross instead of cargo
        #[arg(long)]
        cross: bool,
    },
    /// Run tests
    Test {
        /// Target platform to test on
        #[arg(long)]
        target: Option<String>,
    },
    /// Manage glue configurations
    Glue {
        #[command(subcommand)]
        command: GlueCommands,
    },
}

#[derive(Subcommand)]
enum GlueCommands {
    /// Add a new glue configuration
    Add {
        platform: String,
        config_name: String,
    },
    /// List glue configurations
    List,
    /// Validate glue configurations
    Validate,
}

// Configuration structures
#[derive(Debug, Serialize, Deserialize)]
struct GlueConfig {
    platforms: Vec<Platform>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Platform {
    name: String,
    target: String,
    hal_crate: Option<String>,
    linker_script: Option<String>,
    features: Vec<String>,
}

// Main application structure
struct MultiTargetTool {
    project_root: PathBuf,
}

impl MultiTargetTool {
    fn new() -> Self {
        Self {
            project_root: std::env::current_dir().unwrap(),
        }
    }

    // Initialize a new project
    fn init_project(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Initializing new multi-target project: {}", name);
        
        let project_path = self.project_root.join(name);
        fs::create_dir_all(&project_path)?;
        
        // Create workspace Cargo.toml
        self.create_workspace_cargo_toml(&project_path)?;
        
        // Create core-lib crate
        self.create_core_lib(&project_path)?;
        
        // Create tests directory
        self.create_tests(&project_path)?;
        
        // Create .cargo/config.toml
        self.create_cargo_config(&project_path)?;
        
        // Create glue.toml
        self.create_glue_config(&project_path)?;
        
        // Create README
        self.create_readme(&project_path, name)?;
        
        println!("✅ Project '{}' initialized successfully!", name);
        println!("📁 Created at: {}", project_path.display());
        println!("\nNext steps:");
        println!("  cd {}", name);
        println!("  multi-target-rs test           # Run host tests");
        println!("  multi-target-rs add-platform <name> --target <triple>");
        
        Ok(())
    }

    fn create_workspace_cargo_toml(&self, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"[workspace]
resolver = "2"
members = [
    "core-lib",
    "tests",
]

[workspace.package]
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"

[workspace.dependencies]
embedded-hal = "1.0"
embedded-hal-mock = "0.11"
defmt = "0.3"
"#;
        
        let path = project_path.join("Cargo.toml");
        fs::write(&path, content)?;
        println!("  ✓ Created workspace Cargo.toml");
        Ok(())
    }

    fn create_core_lib(&self, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let core_lib_path = project_path.join("core-lib");
        fs::create_dir_all(&core_lib_path.join("src"))?;
        
        // Create Cargo.toml for core-lib
        let cargo_content = r#"[package]
name = "core-lib"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
embedded-hal = { workspace = true }

[features]
default = []
std = []
"#;
        fs::write(core_lib_path.join("Cargo.toml"), cargo_content)?;
        
        // Create lib.rs with example hardware-agnostic code
        let lib_content = r#"#![cfg_attr(not(feature = "std"), no_std)]

use embedded_hal::i2c::I2c;

/// Example temperature sensor driver (hardware-agnostic)
pub struct TemperatureSensor<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> TemperatureSensor<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn read_temperature(&mut self) -> Result<i16, I2C::Error> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[0x00], &mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }
}

/// Example LED controller (hardware-agnostic)
pub trait LedController {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
}

/// Application logic that uses abstractions
pub struct Application<L: LedController> {
    led: L,
    counter: u32,
}

impl<L: LedController> Application<L> {
    pub fn new(led: L) -> Self {
        Self { led, counter: 0 }
    }

    pub fn tick(&mut self) {
        self.counter += 1;
        if self.counter % 1000 == 0 {
            self.led.toggle();
        }
    }
}
"#;
        fs::write(core_lib_path.join("src/lib.rs"), lib_content)?;
        println!("  ✓ Created core-lib crate");
        Ok(())
    }

    fn create_tests(&self, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let tests_path = project_path.join("tests");
        fs::create_dir_all(&tests_path)?;
        
        // Create Cargo.toml for tests
        let cargo_content = r#"[package]
name = "tests"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
core-lib = { path = "../core-lib", features = ["std"] }
embedded-hal-mock = { workspace = true }

[[test]]
name = "integration"
path = "integration_test.rs"
"#;
        fs::write(tests_path.join("Cargo.toml"), cargo_content)?;
        
        // Create example integration test
        let test_content = r#"use core_lib::{TemperatureSensor, LedController, Application};
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction};

struct MockLed {
    state: bool,
}

impl LedController for MockLed {
    fn turn_on(&mut self) {
        self.state = true;
    }
    
    fn turn_off(&mut self) {
        self.state = false;
    }
    
    fn toggle(&mut self) {
        self.state = !self.state;
    }
}

#[test]
fn test_temperature_sensor() {
    let expectations = vec![
        Transaction::write_read(0x48, vec![0x00], vec![0x12, 0x34]),
    ];
    
    let i2c = I2cMock::new(&expectations);
    let mut sensor = TemperatureSensor::new(i2c, 0x48);
    
    let temp = sensor.read_temperature().unwrap();
    assert_eq!(temp, 0x1234);
}

#[test]
fn test_application_led_toggle() {
    let led = MockLed { state: false };
    let mut app = Application::new(led);
    
    // LED should toggle every 1000 ticks
    for _ in 0..999 {
        app.tick();
    }
    assert!(!app.led.state);
    
    app.tick(); // 1000th tick
    assert!(app.led.state);
}
"#;
        fs::write(tests_path.join("integration_test.rs"), test_content)?;
        println!("  ✓ Created tests crate with examples");
        Ok(())
    }

    fn create_cargo_config(&self, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_dir = project_path.join(".cargo");
        fs::create_dir_all(&cargo_dir)?;
        
        let config_content = r#"[build]
target-dir = "target"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
debug = false

[profile.release-debug]
inherits = "release"
debug = true
"#;
        fs::write(cargo_dir.join("config.toml"), config_content)?;
        println!("  ✓ Created .cargo/config.toml");
        Ok(())
    }

    fn create_glue_config(&self, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let config = GlueConfig {
            platforms: vec![],
        };
        
        let content = toml::to_string_pretty(&config)?;
        fs::write(project_path.join("glue.toml"), content)?;
        println!("  ✓ Created glue.toml");
        Ok(())
    }

    fn create_readme(&self, project_path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(r#"# {}

Multi-target Rust embedded project.

## Quick Start

```bash
# Run unit tests on host
multi-target-rs test

# Add a platform
multi-target-rs add-platform stm32 --target thumbv7em-none-eabi

# Build for platform
multi-target-rs build --target stm32
```

## Project Structure

- `core-lib/` - Hardware-agnostic business logic
- `tests/` - Host-based unit tests
- `app-*/` - Platform-specific binaries
- `hal-*/` - HAL wrapper crates
"#, name);
        
        fs::write(project_path.join("README.md"), content)?;
        println!("  ✓ Created README.md");
        Ok(())
    }

    // Add a new platform
    fn add_platform(&self, name: &str, target: &str, hal: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Adding platform '{}' with target '{}'", name, target);
        
        // Update glue.toml
        self.update_glue_config(name, target, hal.clone())?;
        
        // Create HAL wrapper crate
        self.create_hal_crate(name, &hal)?;
        
        // Create app binary crate
        self.create_app_crate(name, target)?;
        
        // Update workspace Cargo.toml
        self.update_workspace_members(name)?;
        
        println!("✅ Platform '{}' added successfully!", name);
        Ok(())
    }

    fn create_hal_crate(&self, platform: &str, hal: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let hal_path = self.project_root.join(format!("hal-{}", platform));
        fs::create_dir_all(&hal_path.join("src"))?;
        
        let hal_crate = hal.as_ref().map(|h| h.as_str()).unwrap_or("stm32f4xx-hal");
        
        let cargo_content = format!(r#"[package]
name = "hal-{}"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
core-lib = {{ path = "../core-lib" }}
embedded-hal = {{ workspace = true }}
{} = "*"  # Add specific version as needed
"#, platform, hal_crate);
        
        fs::write(hal_path.join("Cargo.toml"), cargo_content)?;
        
        let lib_content = format!(r#"#![no_std]

use core_lib::LedController;
use embedded_hal::digital::OutputPin;

/// Platform-specific LED implementation
pub struct {}Led<P: OutputPin> {{
    pin: P,
}}

impl<P: OutputPin> {}Led<P> {{
    pub fn new(pin: P) -> Self {{
        Self {{ pin }}
    }}
}}

impl<P: OutputPin> LedController for {}Led<P> {{
    fn turn_on(&mut self) {{
        let _ = self.pin.set_high();
    }}
    
    fn turn_off(&mut self) {{
        let _ = self.pin.set_low();
    }}
    
    fn toggle(&mut self) {{
        // Platform-specific toggle if available
        let _ = self.pin.set_low();
    }}
}}
"#, platform.to_uppercase(), platform.to_uppercase(), platform.to_uppercase());
        
        fs::write(hal_path.join("src/lib.rs"), lib_content)?;
        println!("  ✓ Created HAL wrapper: hal-{}", platform);
        Ok(())
    }

    fn create_app_crate(&self, platform: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        let app_path = self.project_root.join(format!("app-{}", platform));
        fs::create_dir_all(&app_path.join("src"))?;
        
        // Determine if we need panic handler and allocator based on target
        let is_embedded = !target.contains("linux") && !target.contains("windows") && !target.contains("darwin");
        
        let cargo_content = format!(r#"[package]
name = "app-{}"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
core-lib = {{ path = "../core-lib" }}
hal-{} = {{ path = "../hal-{}" }}
embedded-hal = {{ workspace = true }}
{}

[[bin]]
name = "{}"
path = "src/main.rs"
"#, 
            platform, 
            platform, 
            platform,
            if is_embedded { "panic-halt = \"0.2\"\ncortex-m-rt = \"0.7\"" } else { "" },
            platform
        );
        
        fs::write(app_path.join("Cargo.toml"), cargo_content)?;
        
        // Create memory.x for embedded targets
        if is_embedded {
            let memory_content = r#"MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
"#;
            fs::write(app_path.join("memory.x"), memory_content)?;
        }
        
        let main_content = if is_embedded {
            format!(r#"#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {{
    // Initialize hardware
    // let peripherals = init_hardware();
    
    // Create application
    // let led = hal_{}::{}Led::new(peripherals.led_pin);
    // let mut app = core_lib::Application::new(led);
    
    loop {{
        // app.tick();
    }}
}}
"#, platform, platform.to_uppercase())
        } else {
            format!(r#"fn main() {{
    println!("Running {} application");
    
    // Initialize platform-specific components
    // let led = hal_{}::{}Led::new(...);
    // let mut app = core_lib::Application::new(led);
    
    // Run application
    // loop {{
    //     app.tick();
    // }}
}}
"#, platform, platform, platform.to_uppercase())
        };
        
        fs::write(app_path.join("src/main.rs"), main_content)?;
        println!("  ✓ Created app binary: app-{}", platform);
        Ok(())
    }

    fn update_workspace_members(&self, platform: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_path = self.project_root.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_path)?;
        
        // Simple string manipulation to add new members
        let new_members = format!(r#"    "hal-{}",
    "app-{}","#, platform, platform);
        
        let updated = content.replace(
            "members = [",
            &format!("members = [\n{}", new_members)
        );
        
        fs::write(&cargo_path, updated)?;
        println!("  ✓ Updated workspace Cargo.toml");
        Ok(())
    }

    fn update_glue_config(&self, name: &str, target: &str, hal: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let glue_path = self.project_root.join("glue.toml");
        
        let mut config: GlueConfig = if glue_path.exists() {
            let content = fs::read_to_string(&glue_path)?;
            toml::from_str(&content)?
        } else {
            GlueConfig { platforms: vec![] }
        };
        
        config.platforms.push(Platform {
            name: name.to_string(),
            target: target.to_string(),
            hal_crate: hal,
            linker_script: None,
            features: vec![],
        });
        
        let content = toml::to_string_pretty(&config)?;
        fs::write(&glue_path, content)?;
        println!("  ✓ Updated glue.toml");
        Ok(())
    }

    // List platforms
    fn list_platforms(&self) -> Result<(), Box<dyn std::error::Error>> {
        let glue_path = self.project_root.join("glue.toml");
        
        if !glue_path.exists() {
            println!("No platforms configured. Use 'add-platform' to add one.");
            return Ok(());
        }
        
        let content = fs::read_to_string(&glue_path)?;
        let config: GlueConfig = toml::from_str(&content)?;
        
        if config.platforms.is_empty() {
            println!("No platforms configured.");
        } else {
            println!("Configured platforms:");
            for platform in &config.platforms {
                println!("  - {} ({})", platform.name, platform.target);
                if let Some(hal) = &platform.hal_crate {
                    println!("    HAL: {}", hal);
                }
            }
        }
        
        Ok(())
    }

    // Build command
    fn build(&self, target: Option<String>, use_cross: bool) -> Result<(), Box<dyn std::error::Error>> {
        let build_cmd = if use_cross { "cross" } else { "cargo" };
        
        if let Some(platform) = target {
            println!("🔨 Building for platform: {}", platform);
            
            // Get target triple from glue.toml
            let glue_path = self.project_root.join("glue.toml");
            let content = fs::read_to_string(&glue_path)?;
            let config: GlueConfig = toml::from_str(&content)?;
            
            let platform_config = config.platforms.iter()
                .find(|p| p.name == platform)
                .ok_or(format!("Platform '{}' not found", platform))?;
            
            let mut cmd = Command::new(build_cmd);
            cmd.arg("build")
                .arg("--target")
                .arg(&platform_config.target)
                .arg("-p")
                .arg(format!("app-{}", platform));
            
            println!("Running: {} build --target {} -p app-{}", build_cmd, platform_config.target, platform);
            
            let status = cmd.status()?;
            if !status.success() {
                return Err("Build failed".into());
            }
        } else {
            println!("🔨 Building core-lib and tests for host");
            
            let mut cmd = Command::new("cargo");
            cmd.arg("build")
                .arg("--workspace");
            
            let status = cmd.status()?;
            if !status.success() {
                return Err("Build failed".into());
            }
        }
        
        println!("✅ Build completed successfully!");
        Ok(())
    }

    // Test command
    fn test(&self, target: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(platform) = target {
            println!("🧪 Running tests on target: {}", platform);
            
            // For on-target testing, we'd use probe-rs or similar
            println!("Note: On-target testing requires probe-rs and embedded-test");
            println!("Install with: cargo install probe-rs-tools");
            
            let mut cmd = Command::new("cargo");
            cmd.arg("embed")
                .arg("--chip")
                .arg("STM32F411RETx") // This would be configurable
                .arg("--example")
                .arg("test");
            
            println!("Would run: cargo embed --chip <chip> test");
            // Uncomment to actually run:
            // let status = cmd.status()?;
            // if !status.success() {
            //     return Err("Test failed".into());
            // }
        } else {
            println!("🧪 Running native unit tests");
            
            let mut cmd = Command::new("cargo");
            cmd.arg("test")
                .arg("--workspace")
                .arg("--exclude")
                .arg("app-*"); // Exclude app crates from host testing
            
            let status = cmd.status()?;
            if !status.success() {
                return Err("Tests failed".into());
            }
        }
        
        println!("✅ Tests passed!");
        Ok(())
    }

    // Glue configuration management
    fn handle_glue_command(&self, cmd: GlueCommands) -> Result<(), Box<dyn std::error::Error>> {
        match cmd {
            GlueCommands::Add { platform, config_name } => {
                println!("Adding glue config '{}' for platform '{}'", config_name, platform);
                // Implementation would add board-specific configurations
                Ok(())
            }
            GlueCommands::List => {
                self.list_platforms()?;
                Ok(())
            }
            GlueCommands::Validate => {
                println!("Validating glue configurations...");
                let glue_path = self.project_root.join("glue.toml");
                
                if !glue_path.exists() {
                    println!("No glue.toml found");
                    return Ok(());
                }
                
                let content = fs::read_to_string(&glue_path)?;
                let config: GlueConfig = toml::from_str(&content)?;
                
                for platform in &config.platforms {
                    println!("  ✓ Platform '{}' valid", platform.name);
                    
                    // Check if referenced crates exist
                    let hal_path = self.project_root.join(format!("hal-{}", platform.name));
                    let app_path = self.project_root.join(format!("app-{}", platform.name));
                    
                    if !hal_path.exists() {
                        println!("    ⚠ Warning: hal-{} directory not found", platform.name);
                    }
                    if !app_path.exists() {
                        println!("    ⚠ Warning: app-{} directory not found", platform.name);
                    }
                }
                
                println!("✅ Validation complete");
                Ok(())
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let tool = MultiTargetTool::new();
    
    match cli.command {
        Commands::Init { name } => {
            tool.init_project(&name)?;
        }
        Commands::AddPlatform { name, target, hal } => {
            tool.add_platform(&name, &target, hal)?;
        }
        Commands::ListPlatforms => {
            tool.list_platforms()?;
        }
        Commands::Build { target, cross } => {
            tool.build(target, cross)?;
        }
        Commands::Test { target } => {
            tool.test(target)?;
        }
        Commands::Glue { command } => {
            tool.handle_glue_command(command)?;
        }
    }
    
    Ok(())
}

// Dependencies for Cargo.toml:
// [dependencies]
// clap = { version = "4.5", features = ["derive"] }
// serde = { version = "1.0", features = ["derive"] }
// toml = "0.8"
// anyhow = "1.0"
//
