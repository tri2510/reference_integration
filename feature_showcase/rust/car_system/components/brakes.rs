//! Brakes component - manages brake pressure and application
//! Demonstrates S-CORE patterns:
//! - State management
//! - Input validation
//! - Gradual state changes (pressure decay)
//! - Message publishing (Phase 3)

use crate::components::{CarComponent, ComponentState, CarMessage};

/// Brakes component - manages the car's braking system
pub struct BrakesComponent {
    state: ComponentState,
    applied: bool,
    pressure: u8, // 0-100%
}

impl BrakesComponent {
    /// Create a new brakes component
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            applied: false,
            pressure: 0,
        }
    }

    /// Apply brakes with specified pressure (0-100%)
    pub fn apply(&mut self, pressure: u8) -> Result<(), String> {
        if pressure > 100 {
            return Err("Pressure cannot exceed 100%".to_string());
        }

        self.applied = true;
        self.pressure = pressure;
        println!("  🛞 Brakes: Applied at {}% pressure", pressure);
        Ok(())
    }

    /// Release brakes
    pub fn release(&mut self) {
        if self.applied {
            println!("  🛞 Brakes: Releasing");
            self.applied = false;
        }
    }

    /// Get current brake pressure
    pub fn get_pressure(&self) -> u8 {
        self.pressure
    }

    /// Check if brakes are applied
    pub fn is_applied(&self) -> bool {
        self.applied
    }

    /// Get messages to publish (Phase 3: Communication)
    pub fn get_messages(&self) -> Vec<CarMessage> {
        let mut messages = Vec::new();

        // Report brake pressure changes
        if self.pressure > 0 {
            messages.push(CarMessage::BrakePressureChange {
                pressure: self.pressure,
            });
        }

        messages
    }
}

impl CarComponent for BrakesComponent {
    fn name(&self) -> &str {
        "Brakes"
    }

    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Brakes: Initializing component...");
        self.state = ComponentState::Initializing;

        // Simulate initialization checks
        println!("  🔍 Brakes: Checking brake fluid... OK");
        println!("  🔍 Brakes: Checking brake pads... OK");
        println!("  🔍 Brakes: Checking ABS system... OK");

        self.state = ComponentState::Online;
        println!("✅ Brakes: Initialized (state: {})", self.state);
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        // Brake pressure slowly releases if not actively applied
        // This simulates gradual pressure decay
        if !self.applied && self.pressure > 0 {
            self.pressure = self.pressure.saturating_sub(5);
            if self.pressure == 0 {
                println!("  🛞 Brakes: Fully released");
            }
        }

        Ok(())
    }

    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
