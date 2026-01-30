//! State machine trait and implementations
//! This demonstrates S-CORE's state management patterns

use std::fmt;

/// State machine trait - enforces valid state transitions
pub trait StateMachine: Sized {
    type State: std::fmt::Debug;

    /// Get current state
    fn current_state(&self) -> &Self::State;

    /// Check if transition is valid
    fn can_transition_to(&self, new_state: &Self::State) -> bool;

    /// Transition to new state (returns error if invalid)
    fn transition(&mut self, new_state: Self::State) -> Result<(), String> {
        if !self.can_transition_to(&new_state) {
            return Err(format!(
                "Invalid transition: {:?} → {:?}",
                self.current_state(),
                new_state
            ));
        }
        self.set_state(new_state);
        Ok(())
    }

    /// Internal method to set state (after validation)
    fn set_state(&mut self, new_state: Self::State);
}

/// Engine state machine with valid transitions
#[derive(Debug, Clone, PartialEq)]
pub enum EngineStateMachine {
    Off,
    Starting,
    Running,
    Stopping,
}

impl EngineStateMachine {
    /// Get all valid transitions from current state
    pub fn valid_transitions(&self) -> Vec<EngineStateMachine> {
        match self {
            EngineStateMachine::Off => vec![EngineStateMachine::Starting],
            EngineStateMachine::Starting => vec![EngineStateMachine::Running, EngineStateMachine::Off],
            EngineStateMachine::Running => vec![EngineStateMachine::Stopping],
            EngineStateMachine::Stopping => vec![EngineStateMachine::Off],
        }
    }

    /// Check if transition is valid
    pub fn can_transition_to(&self, new_state: &EngineStateMachine) -> bool {
        self.valid_transitions().contains(new_state)
    }

    /// Transition with validation
    pub fn transition(&self) -> Result<EngineStateMachine, String> {
        match self {
            EngineStateMachine::Off => Ok(EngineStateMachine::Starting),
            EngineStateMachine::Starting => Ok(EngineStateMachine::Running),
            EngineStateMachine::Running => Ok(EngineStateMachine::Stopping),
            EngineStateMachine::Stopping => Ok(EngineStateMachine::Off),
        }
    }
}

impl fmt::Display for EngineStateMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineStateMachine::Off => write!(f, "OFF"),
            EngineStateMachine::Starting => write!(f, "STARTING"),
            EngineStateMachine::Running => write!(f, "RUNNING"),
            EngineStateMachine::Stopping => write!(f, "STOPPING"),
        }
    }
}
