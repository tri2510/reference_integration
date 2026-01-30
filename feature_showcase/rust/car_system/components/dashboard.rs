//! Dashboard component - displays all system states
//! Demonstrates S-CORE patterns:
//! - Aggregating state from multiple components
//! - Display formatting and status reporting
//! - Warning management

use crate::components::{CarComponent, ComponentState};

/// Dashboard component - displays all car system information
pub struct DashboardComponent {
    state: ComponentState,
    speed: u8,           // km/h
    fuel_level: u8,      // 0-100%
    warnings: Vec<String>,
    odometer: f32,       // km
}

impl DashboardComponent {
    /// Create a new dashboard component
    pub fn new() -> Self {
        Self {
            state: ComponentState::Offline,
            speed: 0,
            fuel_level: 100,
            warnings: Vec::new(),
            odometer: 0.0,
        }
    }

    /// Set current speed
    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed.min(200); // Max speed cap
    }

    /// Set fuel level
    pub fn set_fuel_level(&mut self, level: u8) {
        self.fuel_level = level.min(100);
    }

    /// Add a warning message
    pub fn add_warning(&mut self, warning: String) {
        // Avoid duplicate warnings
        if !self.warnings.contains(&warning) {
            self.warnings.push(warning);
        }
    }

    /// Clear all warnings
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    /// Get current speed
    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    /// Get fuel level
    pub fn get_fuel_level(&self) -> u8 {
        self.fuel_level
    }

    /// Update odometer
    pub fn update_odometer(&mut self, km: f32) {
        self.odometer += km;
    }

    /// Display dashboard with engine status
    pub fn display(&self, rpm: u32, temp: f32, brake_pressure: u8, steering_angle: i16) {
        println!("┌────────────────────────────────────────────────────────────┐");
        println!("│                    🚗 CAR DASHBOARD                         │");
        println!("├────────────────────────────────────────────────────────────┤");
        println!("│ Speed:        {:>3} km/h     Fuel:        {:>3}%           │",
                 self.speed, self.fuel_level);
        println!("│ Engine RPM:   {:>4}         Temp:        {:>4.1}°C        │",
                 rpm, temp);
        println!("│ Brake Press:  {:>3}%         Steering:    {:>4}° ({:<6}) │",
                 brake_pressure, steering_angle,
                 if steering_angle > 10 { "RIGHT" }
                 else if steering_angle < -10 { "LEFT" }
                 else { "CENTER" });
        println!("│ Odometer:     {:>8.1} km                                        │",
                 self.odometer);
        println!("├────────────────────────────────────────────────────────────┤");

        if !self.warnings.is_empty() {
            println!("│ ⚠️  WARNINGS:                                                   │");
            for warning in &self.warnings {
                println!("│   • {}{:.<54}│", warning, "");
            }
        } else {
            println!("│ ✅ All systems OK                                             │");
        }

        println!("└────────────────────────────────────────────────────────────┘");
    }
}

impl CarComponent for DashboardComponent {
    fn name(&self) -> &str {
        "Dashboard"
    }

    fn initialize(&mut self) -> Result<(), String> {
        println!("🔧 Dashboard: Initializing component...");
        self.state = ComponentState::Initializing;

        // Simulate initialization
        println!("  🔍 Dashboard: Testing display... OK");
        println!("  🔍 Dashboard: Checking sensors... OK");

        self.state = ComponentState::Online;
        println!("✅ Dashboard: Initialized (state: {})", self.state);
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        // Low fuel warning
        if self.fuel_level < 20 && self.fuel_level > 0 {
            let warning = format!("Low fuel ({}%)", self.fuel_level);
            self.add_warning(warning);
        }

        // High speed warning
        if self.speed > 120 {
            self.add_warning("High speed - drive carefully".to_string());
        }

        Ok(())
    }

    fn get_state(&self) -> ComponentState {
        self.state.clone()
    }
}
