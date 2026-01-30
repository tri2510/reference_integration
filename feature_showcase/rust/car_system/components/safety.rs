//! Safety monitor and fault handling
//! This demonstrates S-CORE's safety patterns (like ISO 26262)

use std::fmt;

/// Safety warning types
#[derive(Debug, Clone, PartialEq)]
pub enum SafetyWarning {
    SpeedExceeded { current: u8, max: u8 },
    Overheating { current: f32, max: f32 },
    HighRPM { current: u32, max: u32 },
    LowFuel { level: u8 },
    BrakePressureTooHigh { pressure: u8 },
    EngineStateInvalid { state: String },
}

impl fmt::Display for SafetyWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafetyWarning::SpeedExceeded { current, max } => {
                write!(f, "⚠️ SPEED EXCEEDED: {} km/h (max: {} km/h)", current, max)
            }
            SafetyWarning::Overheating { current, max } => {
                write!(f, "⚠️ ENGINE OVERHEATING: {:.1}°C (max: {:.1}°C)", current, max)
            }
            SafetyWarning::HighRPM { current, max } => {
                write!(f, "⚠️ HIGH RPM: {} (max: {})", current, max)
            }
            SafetyWarning::LowFuel { level } => {
                write!(f, "⚠️ LOW FUEL: {}%", level)
            }
            SafetyWarning::BrakePressureTooHigh { pressure } => {
                write!(f, "⚠️ BRAKE PRESSURE TOO HIGH: {}%", pressure)
            }
            SafetyWarning::EngineStateInvalid { state } => {
                write!(f, "⚠️ ENGINE STATE INVALID: {}", state)
            }
        }
    }
}

/// Safety severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetySeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

impl SafetyWarning {
    /// Get severity level for this warning
    pub fn severity(&self) -> SafetySeverity {
        match self {
            SafetyWarning::SpeedExceeded { current, max } => {
                if *current > *max + 20 { SafetySeverity::Critical }
                else { SafetySeverity::Warning }
            }
            SafetyWarning::Overheating { current, max } => {
                if *current > *max + 10.0 { SafetySeverity::Emergency }
                else if *current > *max { SafetySeverity::Critical }
                else { SafetySeverity::Warning }
            }
            SafetyWarning::HighRPM { current, max } => {
                if *current > *max + 1000 { SafetySeverity::Critical }
                else { SafetySeverity::Warning }
            }
            SafetyWarning::LowFuel { .. } => SafetySeverity::Warning,
            SafetyWarning::BrakePressureTooHigh { .. } => SafetySeverity::Info,
            SafetyWarning::EngineStateInvalid { .. } => SafetySeverity::Emergency,
        }
    }
}

/// Safety monitor - enforces safety limits
pub struct SafetyMonitor {
    pub max_speed: u8,
    pub max_temperature: f32,
    pub max_rpm: u32,
    min_fuel: u8,
    max_brake_pressure: u8,
}

impl SafetyMonitor {
    /// Create a new safety monitor with standard limits
    pub fn new() -> Self {
        Self {
            max_speed: 120,        // km/h
            max_temperature: 95.0, // °C
            max_rpm: 6000,         // RPM
            min_fuel: 15,          // %
            max_brake_pressure: 80, // %
        }
    }

    /// Create with custom limits
    pub fn with_limits(max_speed: u8, max_temp: f32, max_rpm: u32) -> Self {
        Self {
            max_speed,
            max_temperature: max_temp,
            max_rpm,
            ..Self::new()
        }
    }

    /// Check system state and return all safety warnings
    pub fn check(&self, speed: u8, temp: f32, rpm: u32, fuel: u8,
                 brake_pressure: u8, engine_running: bool) -> Vec<SafetyWarning> {
        let mut warnings = Vec::new();

        // Check speed limit
        if speed > self.max_speed {
            warnings.push(SafetyWarning::SpeedExceeded {
                current: speed,
                max: self.max_speed,
            });
        }

        // Check temperature
        if temp > self.max_temperature {
            warnings.push(SafetyWarning::Overheating {
                current: temp,
                max: self.max_temperature,
            });
        }

        // Check RPM
        if rpm > self.max_rpm {
            warnings.push(SafetyWarning::HighRPM {
                current: rpm,
                max: self.max_rpm,
            });
        }

        // Check fuel level
        if fuel < self.min_fuel {
            warnings.push(SafetyWarning::LowFuel { level: fuel });
        }

        // Check brake pressure
        if brake_pressure > self.max_brake_pressure {
            warnings.push(SafetyWarning::BrakePressureTooHigh {
                pressure: brake_pressure,
            });
        }

        // Check engine state validity
        if !engine_running && speed > 0 {
            warnings.push(SafetyWarning::EngineStateInvalid {
                state: "Engine off but car moving".to_string(),
            });
        }

        warnings
    }

    /// Check if system is safe to operate
    pub fn is_safe(&self, warnings: &[SafetyWarning]) -> bool {
        !warnings.iter().any(|w| w.severity() >= SafetySeverity::Critical)
    }
}

impl Default for SafetyMonitor {
    fn default() -> Self {
        Self::new()
    }
}
