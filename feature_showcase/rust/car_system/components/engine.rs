//! Engine component - manages engine state, RPM, and temperature
//! This demonstrates S-CORE patterns:
//! - Component state management
//! - Structured logging
//! - Lifecycle management (initialize -> process -> shutdown)
//! - Message publishing (Phase 3)
//! - State machine with valid transitions (Phase 4)

use crate::components::{CarComponent, ComponentState, CarMessage, ComponentId};
use crate::components::state_machine::EngineStateMachine;

/// Engine-specific states (using state machine)
pub type EngineState = EngineStateMachine;

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

    /// Start the engine (with state machine validation)
    pub fn start(&mut self) -> Result<(), String> {
        // Phase 4: Validate state transition using state machine
        if !self.engine_state.can_transition_to(&EngineState::Starting) {
            return Err(format!(
                "Cannot start engine: invalid transition from {} to STARTING",
                self.engine_state
            ));
        }

        println!("  🔑 Engine: {} → STARTING", self.engine_state);
        self.state = ComponentState::Initializing;
        self.engine_state = EngineState::Starting;

        // Simulate startup delay
        self.rpm = 500;

        // Complete transition to Running
        if !self.engine_state.can_transition_to(&EngineState::Running) {
            return Err(format!(
                "Cannot complete startup: invalid transition from {} to RUNNING",
                self.engine_state
            ));
        }

        println!("  🔑 Engine: STARTING → RUNNING");
        self.state = ComponentState::Online;
        self.running = true;
        self.engine_state = EngineState::Running;
        self.rpm = 800; // Idle RPM

        println!("  ✅ Engine: Started successfully (state: {})", self.engine_state);
        Ok(())
    }

    /// Stop the engine (with state machine validation)
    pub fn stop(&mut self) -> Result<(), String> {
        // Phase 4: Validate state transition using state machine
        if !self.engine_state.can_transition_to(&EngineState::Stopping) {
            return Err(format!(
                "Cannot stop engine: invalid transition from {} to STOPPING",
                self.engine_state
            ));
        }

        println!("  🔑 Engine: {} → STOPPING", self.engine_state);
        self.engine_state = EngineState::Stopping;
        self.running = false;
        self.rpm = 0;

        // Complete transition to Off
        if !self.engine_state.can_transition_to(&EngineState::Off) {
            return Err(format!(
                "Cannot complete shutdown: invalid transition from {} to OFF",
                self.engine_state
            ));
        }

        println!("  🔑 Engine: STOPPING → OFF");
        self.engine_state = EngineState::Off;
        self.state = ComponentState::Offline;

        println!("  ✅ Engine: Stopped (state: {})", self.engine_state);
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

    /// Get current engine state (Phase 4: State machine)
    pub fn get_engine_state(&self) -> &EngineState {
        &self.engine_state
    }

    /// Get messages to publish (Phase 3: Communication)
    /// Returns messages the engine wants to send to other components
    pub fn get_messages(&self) -> Vec<CarMessage> {
        let mut messages = Vec::new();

        // Check for overheating (lowered to 21.0 so it appears during demo)
        if self.running && self.temperature > 21.0 {
            messages.push(CarMessage::EngineOverheating {
                temperature: self.temperature,
            });
        }

        // Report RPM changes
        if self.running {
            messages.push(CarMessage::EngineRpmChange { rpm: self.rpm });
        }

        messages
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
