//! Message types for component communication
//! This defines all events that components can publish/subscribe to

/// Car messages - events that components can send
#[derive(Debug, Clone, PartialEq)]
pub enum CarMessage {
    /// Engine events
    EngineStart,
    EngineStop,
    EngineOverheating { temperature: f32 },
    EngineRpmChange { rpm: u32 },

    /// Brake events
    BrakeApply { pressure: u8 },
    BrakeRelease,
    BrakePressureChange { pressure: u8 },

    /// Steering events
    SteeringTurn { angle: i16 },
    SteeringCenter,

    /// Vehicle events
    SpeedUpdate { km_h: u8 },
    FuelWarning { level: u8 },

    /// System events
    ComponentError { component: String, error: String },
}

impl CarMessage {
    /// Get message type name for logging
    pub fn type_name(&self) -> &str {
        match self {
            CarMessage::EngineStart => "EngineStart",
            CarMessage::EngineStop => "EngineStop",
            CarMessage::EngineOverheating { .. } => "EngineOverheating",
            CarMessage::EngineRpmChange { .. } => "EngineRpmChange",
            CarMessage::BrakeApply { .. } => "BrakeApply",
            CarMessage::BrakeRelease => "BrakeRelease",
            CarMessage::BrakePressureChange { .. } => "BrakePressureChange",
            CarMessage::SteeringTurn { .. } => "SteeringTurn",
            CarMessage::SteeringCenter => "SteeringCenter",
            CarMessage::SpeedUpdate { .. } => "SpeedUpdate",
            CarMessage::FuelWarning { .. } => "FuelWarning",
            CarMessage::ComponentError { .. } => "ComponentError",
        }
    }

    /// Format message for display
    pub fn format(&self) -> String {
        match self {
            CarMessage::EngineStart => "Engine started".to_string(),
            CarMessage::EngineStop => "Engine stopped".to_string(),
            CarMessage::EngineOverheating { temperature } => {
                format!("⚠️ ENGINE OVERHEATING: {}°C", temperature)
            }
            CarMessage::EngineRpmChange { rpm } => format!("Engine RPM: {}", rpm),
            CarMessage::BrakeApply { pressure } => format!("Brakes applied: {}%", pressure),
            CarMessage::BrakeRelease => "Brakes released".to_string(),
            CarMessage::BrakePressureChange { pressure } => {
                format!("Brake pressure: {}%", pressure)
            }
            CarMessage::SteeringTurn { angle } => format!("Steering turned: {}°", angle),
            CarMessage::SteeringCenter => "Steering centered".to_string(),
            CarMessage::SpeedUpdate { km_h } => format!("Speed: {} km/h", km_h),
            CarMessage::FuelWarning { level } => {
                format!("⚠️ LOW FUEL: {}%", level)
            }
            CarMessage::ComponentError { component, error } => {
                format!("❌ ERROR in {}: {}", component, error)
            }
        }
    }
}

/// Component ID for message routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentId {
    Engine,
    Brakes,
    Steering,
    Dashboard,
    CarSystem,
}

impl ComponentId {
    pub fn as_str(&self) -> &str {
        match self {
            ComponentId::Engine => "Engine",
            ComponentId::Brakes => "Brakes",
            ComponentId::Steering => "Steering",
            ComponentId::Dashboard => "Dashboard",
            ComponentId::CarSystem => "CarSystem",
        }
    }
}
