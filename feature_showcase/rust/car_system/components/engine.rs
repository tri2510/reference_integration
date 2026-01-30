//! Engine component - manages engine state, RPM, and temperature
//! This demonstrates S-CORE patterns:
//! - Component state management
//! - Structured logging
//! - Lifecycle management (initialize -> process -> shutdown)

use crate::components::{CarComponent, ComponentState};

/// Engine-specific states
#[derive(Debug, Clone, PartialEq)]
pub enum EngineState {
    Off,
    Starting,
    Running,
    Stopping,
}

/// Engine component - manages the car's engine
pub struct EngineComponent {
    state: ComponentState,
    engine_state: EngineState,
    running: bool,
    rpm: u32,
    temperature: f32,
    cycle_counter: u32,
}

impl EngineComponent {
    /// Create a new engine component
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            engine_state: EngineState::Off,
            running: false,
            rpm: 0,
            temperature: 20.0, // Ambient temperature
            cycle_counter: 0,
        }
    }

    /// Start the engine
    pub fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("Engine already running".to_string());
        }

        println!("  🔑 Engine: Starting ignition sequence...");
        self.state = ComponentState::Initializing;
        self.engine_state = EngineState::Starting;

        // Simulate startup delay
        self.rpm = 500;
        self.state = ComponentState::Online;
        self.running = true;
        self.engine_state = EngineState::Running;
        self.rpm = 800; // Idle RPM

        println!("  ✅ Engine: Started successfully");
        Ok(())
    }

    /// Stop the engine
    pub fn stop(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("Engine not running".to_string());
        }

        println!("  🔑 Engine: Shutting down...");
        self.engine_state = EngineState::Stopping;
        self.running = false;
        self.rpm = 0;
        self.engine_state = EngineState::Off;
        self.state = ComponentState::Offline;

        println!("  ✅ Engine: Stopped");
        Ok(())
    }

    /// Get current RPM
    pub fn get_rpm(&self) -> u32 {
        self.rpm
    }

    /// Get current temperature
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    /// Check if engine is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl CarComponent for EngineComponent {
    fn name(&self) -> &str {
        "Engine"
    }

    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Engine: Initializing component...");
        self.state = ComponentState::Initializing;

        // Simulate initialization checks
        println!("  🔍 Engine: Checking oil level... OK");
        println!("  🔍 Engine: Checking fuel pressure... OK");
        println!("  🔍 Engine: Checking ignition system... OK");

        self.state = ComponentState::Online;
        println!("✅ Engine: Initialized (state: {})", self.state);
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        // Simulate RPM fluctuation when running
        if self.running {
            // Use cycle counter to create pseudo-random fluctuation
            self.cycle_counter = self.cycle_counter.wrapping_add(1);
            let fluctuation = ((self.cycle_counter * 17) % 50) as u32;
            self.rpm = 800 + fluctuation;

            // Slowly increase temperature
            if self.temperature < 90.0 {
                self.temperature += 0.05;
            }
        }

        Ok(())
    }

    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
