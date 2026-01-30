# 📗 Phase 2: Multi-Component Architecture

## 🎯 Goal

Create a multi-component car system with 4 independent components:
- **Engine Component** - Manages engine state, RPM, temperature
- **Brakes Component** - Manages brake pressure, application
- **Steering Component** - Manages steering angle
- **Dashboard Component** - Displays all system states

---

## 📁 File Structure

```
feature_showcase/rust/car_system/
├── main.rs              (Entry point & main loop)
├── components/
│   ├── mod.rs          (Component exports)
│   ├── engine.rs       (Engine implementation)
│   ├── brakes.rs       (Brakes implementation)
│   ├── steering.rs     (Steering implementation)
│   └── dashboard.rs    (Dashboard implementation)
└── BUILD               (Bazel build rules)
```

---

## 🔧 Step-by-Step Implementation

### Step 1: Create Directory Structure

```bash
cd /home/htr1hc/01_PJNE/06_score
mkdir -p feature_showcase/rust/car_system/components
```

### Step 2: Create Component Module (components/mod.rs)

```rust
//! Car component definitions

mod engine;
mod brakes;
mod steering;
mod dashboard;

pub use engine::EngineComponent;
pub use brakes::BrakesComponent;
pub use steering::SteeringComponent;
pub use dashboard::DashboardComponent;

/// Common component trait
pub trait CarComponent {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> Result<(), String>;
    fn process(&mut self) -> Result<(), String>;
    fn get_state(&self) -> ComponentState;
}

/// Component state
#[derive(Debug, Clone)]
pub enum ComponentState {
    Offline,
    Initializing,
    Online,
    Error(String),
}
```

### Step 3: Create Engine Component (components/engine.rs)

```rust
//! Engine component

use crate::components::{CarComponent, ComponentState};

pub struct EngineComponent {
    state: ComponentState,
    running: bool,
    rpm: u32,
    temperature: f32,
}

impl EngineComponent {
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            running: false,
            rpm: 0,
            temperature: 20.0,
        }
    }
    
    pub fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("Engine already running".to_string());
        }
        
        self.state = ComponentState::Initializing;
        self.running = true;
        self.rpm = 800; // Idle RPM
        self.state = ComponentState::Online;
        
        Ok(())
    }
    
    pub fn stop(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("Engine not running".to_string());
        }
        
        self.running = false;
        self.rpm = 0;
        self.state = ComponentState::Offline;
        
        Ok(())
    }
    
    pub fn get_rpm(&self) -> u32 {
        self.rpm
    }
    
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

impl CarComponent for EngineComponent {
    fn name(&self) -> &str {
        "Engine"
    }
    
    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Engine: Initializing...");
        self.state = ComponentState::Online;
        Ok(())
    }
    
    fn process(&mut self) -> Result<(), String> {
        if self.running {
            // Simulate RPM fluctuation
            self.rpm = 800 + (rand::random::<u32>() % 50);
            self.temperature += 0.1;
        }
        Ok(())
    }
    
    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
```

### Step 4: Create Brakes Component (components/brakes.rs)

```rust
//! Brakes component

use crate::components::{CarComponent, ComponentState};

pub struct BrakesComponent {
    state: ComponentState,
    applied: bool,
    pressure: u8, // 0-100%
}

impl BrakesComponent {
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            applied: false,
            pressure: 0,
        }
    }
    
    pub fn apply(&mut self, pressure: u8) -> Result<(), String> {
        if pressure > 100 {
            return Err("Pressure cannot exceed 100%".to_string());
        }
        
        self.applied = true;
        self.pressure = pressure;
        Ok(())
    }
    
    pub fn release(&mut self) {
        self.applied = false;
        self.pressure = 0;
    }
    
    pub fn get_pressure(&self) -> u8 {
        self.pressure
    }
}

impl CarComponent for BrakesComponent {
    fn name(&self) -> &str {
        "Brakes"
    }
    
    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Brakes: Initializing...");
        self.state = ComponentState::Online;
        Ok(())
    }
    
    fn process(&mut self) -> Result<(), String> {
        // Brake pressure slowly releases if not actively applied
        if !self.applied && self.pressure > 0 {
            self.pressure = self.pressure.saturating_sub(1);
        }
        Ok(())
    }
    
    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
```

### Step 5: Create Steering Component (components/steering.rs)

```rust
//! Steering component

use crate::components::{CarComponent, ComponentState};

pub struct SteeringComponent {
    state: ComponentState,
    angle: i16, // -90 to +90 degrees
}

impl SteeringComponent {
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            angle: 0,
        }
    }
    
    pub fn turn(&mut self, angle: i16) -> Result<(), String> {
        if angle < -90 || angle > 90 {
            return Err("Angle must be between -90 and +90".to_string());
        }
        
        self.angle = angle;
        Ok(())
    }
    
    pub fn center(&mut self) {
        self.angle = 0;
    }
    
    pub fn get_angle(&self) -> i16 {
        self.angle
    }
}

impl CarComponent for SteeringComponent {
    fn name(&self) -> &str {
        "Steering"
    }
    
    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Steering: Initializing...");
        self.state = ComponentState::Online;
        Ok(())
    }
    
    fn process(&mut self) -> Result<(), String> {
        // Slowly return to center
        if self.angle > 0 {
            self.angle -= 1;
        } else if self.angle < 0 {
            self.angle += 1;
        }
        Ok(())
    }
    
    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
```

### Step 6: Create Dashboard Component (components/dashboard.rs)

```rust
//! Dashboard component

use crate::components::{CarComponent, ComponentState};

pub struct DashboardComponent {
    state: ComponentState,
    speed: u8,
    warnings: Vec<String>,
}

impl DashboardComponent {
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            speed: 0,
            warnings: Vec::new(),
        }
    }
    
    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }
    
    pub fn display(&self) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🚗 CAR DASHBOARD");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Speed: {} km/h", self.speed);
        
        if !self.warnings.is_empty() {
            println!();
            println!("⚠️  WARNINGS:");
            for warning in &self.warnings {
                println!("  - {}", warning);
            }
        }
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

impl CarComponent for DashboardComponent {
    fn name(&self) -> &str {
        "Dashboard"
    }
    
    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Dashboard: Initializing...");
        self.state = ComponentState::Online;
        Ok(())
    }
    
    fn process(&mut self) -> Result<(), String> {
        // Update display
        self.display();
        Ok(())
    }
    
    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
```

### Step 7: Create Main Entry Point (main.rs)

```rust
//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// SPDX-License-Identifier: Apache-2.0
//

mod components;

use components::*;

fn main() -> Result<(), String> {
    println!("🚗 S-CORE Car System - Phase 2");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Create components
    let mut engine = EngineComponent::new();
    let mut brakes = BrakesComponent::new();
    let mut steering = SteeringComponent::new();
    let mut dashboard = DashboardComponent::new();
    
    // Initialize all components
    println!();
    println!("🔧 Initializing components...");
    engine.initialize()?;
    brakes.initialize()?;
    steering.initialize()?;
    dashboard.initialize()?;
    
    // Start engine
    println!();
    println!("🔑 Starting engine...");
    engine.start()?;
    println!("✅ Engine running at {} RPM", engine.get_rpm());
    
    // Simulate driving
    println!();
    println!("🚗 Simulating driving...");
    
    for i in 1..=5 {
        println!();
        println!("📍 Cycle {}", i);
        
        // Update dashboard
        dashboard.set_speed(i * 20);
        
        // Process all components
        engine.process()?;
        brakes.process()?;
        steering.process()?;
        dashboard.process()?;
        
        // Turn steering
        if i == 2 {
            steering.turn(30)?;
            println!("🔄 Turning steering 30°");
        }
        
        // Apply brakes
        if i == 4 {
            brakes.apply(50)?;
            println!("🛞 Applying brakes 50%");
        }
        
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    // Shutdown
    println!();
    println!("🛑 Shutting down...");
    engine.stop()?;
    println!("✅ Engine stopped");
    
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Phase 2 complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}
```

### Step 8: Update BUILD File

```bash
cat >> feature_showcase/rust/BUILD << 'EOF'

rust_binary(
    name = "car_system_example",
    srcs = glob(["car_system/**/*.rs"]),
    visibility = ["//visibility:public"],
)
