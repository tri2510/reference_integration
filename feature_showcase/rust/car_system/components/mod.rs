//! Car component definitions
//! This module defines the common interface that all car components implement

mod engine;
mod brakes;
mod steering;
mod dashboard;
mod messages;
mod message_bus;
mod state_machine;
mod event_loop;
mod safety;
mod workflow;
mod system;

pub use engine::EngineComponent;
pub use brakes::BrakesComponent;
pub use steering::SteeringComponent;
pub use dashboard::DashboardComponent;
pub use messages::{CarMessage, ComponentId};
pub use message_bus::MessageBus;
pub use state_machine::{EngineStateMachine, StateMachine};
pub use event_loop::{EventLoop, EventLoopConfig};
pub use safety::{SafetyMonitor, SafetyWarning, SafetySeverity};
pub use workflow::{Workflow, WorkflowStep, WorkflowBuilder};
pub use system::CarSystem;

/// Common component trait - all car components must implement this
/// This mirrors S-CORE's component-based architecture where each component
/// has a well-defined lifecycle and behavior
pub trait CarComponent {
    /// Returns the component name for logging
    fn name(&self) -> &str;

    /// Initialize the component - called once at startup
    /// Similar to S-CORE component initialization
    fn initialize(&mut self) -> Result<(), String>;

    /// Process the component - called repeatedly during operation
    /// Similar to S-CORE's process loop
    fn process(&mut self) -> Result<(), String>;

    /// Get the current state of the component
    /// Similar to S-CORE's state management
    fn get_state(&self) -> ComponentState;
}

/// Component state enum - represents the lifecycle state
/// Similar to S-CORE's component state management
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentState {
    /// Component is offline/not initialized
    Offline,
    /// Component is initializing
    Initializing,
    /// Component is online and operational
    Online,
    /// Component encountered an error
    Error(String),
}

impl std::fmt::Display for ComponentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentState::Offline => write!(f, "OFFLINE"),
            ComponentState::Initializing => write!(f, "INITIALIZING"),
            ComponentState::Online => write!(f, "ONLINE"),
            ComponentState::Error(msg) => write!(f, "ERROR: {}", msg),
        }
    }
}
