//! Steering component - manages steering angle
//! Demonstrates S-CORE patterns:
//! - Input validation (angle bounds checking)
//! - Automatic state correction (return to center)
//! - Message publishing (Phase 3)

use crate::components::{CarComponent, ComponentState, CarMessage};

/// Steering component - manages the car's steering system
pub struct SteeringComponent {
    state: ComponentState,
    angle: i16, // -90 to +90 degrees (negative = left, positive = right)
}

impl SteeringComponent {
    /// Create a new steering component
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            angle: 0,
        }
    }

    /// Turn steering wheel to specified angle
    /// Negative values turn left, positive values turn right
    pub fn turn(&mut self, angle: i16) -> Result<(), String> {
        if angle < -90 || angle > 90 {
            return Err("Angle must be between -90 and +90 degrees".to_string());
        }

        let direction = if angle > self.angle {
            "right"
        } else if angle < self.angle {
            "left"
        } else {
            "centered"
        };

        self.angle = angle;
        println!("  🔄 Steering: Turn {} to {}°", direction, angle);
        Ok(())
    }

    /// Center the steering wheel
    pub fn center(&mut self) {
        if self.angle != 0 {
            println!("  🔄 Steering: Returning to center");
            self.angle = 0;
        }
    }

    /// Get current steering angle
    pub fn get_angle(&self) -> i16 {
        self.angle
    }

    /// Get direction as string
    pub fn get_direction(&self) -> &'static str {
        if self.angle > 10 {
            "RIGHT"
        } else if self.angle < -10 {
            "LEFT"
        } else {
            "CENTER"
        }
    }

    /// Get messages to publish (Phase 3: Communication)
    pub fn get_messages(&self) -> Vec<CarMessage> {
        let mut messages = Vec::new();

        // Report steering angle when not centered
        if self.angle != 0 {
            messages.push(CarMessage::SteeringTurn { angle: self.angle });
        }

        messages
    }
}

impl CarComponent for SteeringComponent {
    fn name(&self) -> &str {
        "Steering"
    }

    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Steering: Initializing component...");
        self.state = ComponentState::Initializing;

        // Simulate initialization checks
        println!("  🔍 Steering: Checking power steering... OK");
        println!("  🔍 Steering: Calibrating center position... OK");

        self.state = ComponentState::Online;
        println!("✅ Steering: Initialized (state: {})", self.state);
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        // Slowly return to center (power steering assist)
        // This simulates automatic centering behavior
        const CENTERING_RATE: i16 = 2;

        if self.angle > 0 {
            self.angle = (self.angle - CENTERING_RATE).max(0);
        } else if self.angle < 0 {
            self.angle = (self.angle + CENTERING_RATE).min(0);
        }

        Ok(())
    }

    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
