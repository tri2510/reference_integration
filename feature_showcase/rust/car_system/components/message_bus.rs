//! Message bus for component communication
//! This is the central hub that routes messages between components
//! Similar to S-CORE's communication module

use super::messages::{CarMessage, ComponentId};
use std::collections::{HashMap, VecDeque};

/// Message bus - central communication hub
/// Components publish messages, and subscribed components receive them
pub struct MessageBus {
    /// Message queues for each component
    queues: HashMap<ComponentId, VecDeque<CarMessage>>,
    /// Subscriptions: which component wants which message types
    subscriptions: HashMap<ComponentId, bool>, // true = subscribe to all
}

impl MessageBus {
    /// Create a new message bus
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
            subscriptions: HashMap::new(),
        }
    }

    /// Register a component (create its message queue)
    pub fn register_component(&mut self, component_id: ComponentId) {
        self.queues.entry(component_id).or_insert_with(VecDeque::new);
        println!("  📡 MessageBus: Registered {}", component_id.as_str());
    }

    /// Subscribe a component to all messages
    pub fn subscribe_all(&mut self, component_id: ComponentId) {
        self.subscriptions.insert(component_id, true);
        println!("  📡 MessageBus: {} subscribed to ALL messages", component_id.as_str());
    }

    /// Publish a message from a component
    /// The message bus routes it to all subscribed components
    pub fn publish(&mut self, from: ComponentId, message: CarMessage) {
        println!("  📨 [{}] → {}", from.as_str(), message.format());

        // Add message to all subscribers' queues
        for (component_id, _) in &self.subscriptions {
            if *component_id != from {
                // Don't send back to sender
                if let Some(queue) = self.queues.get_mut(component_id) {
                    queue.push_back(message.clone());
                }
            }
        }
    }

    /// Check if a component has pending messages
    pub fn has_messages(&self, component_id: ComponentId) -> bool {
        self.queues
            .get(&component_id)
            .map(|q| !q.is_empty())
            .unwrap_or(false)
    }

    /// Get the number of pending messages for a component
    pub fn pending_count(&self, component_id: ComponentId) -> usize {
        self.queues
            .get(&component_id)
            .map(|q| q.len())
            .unwrap_or(0)
    }

    /// Receive next message for a component (blocking)
    pub fn receive(&mut self, component_id: ComponentId) -> Option<CarMessage> {
        self.queues.get_mut(&component_id)?.pop_front()
    }

    /// Receive all pending messages for a component
    pub fn receive_all(&mut self, component_id: ComponentId) -> Vec<CarMessage> {
        let messages = self.queues.get_mut(&component_id);
        if let Some(queue) = messages {
            let count = queue.len();
            let mut result = Vec::with_capacity(count);
            for _ in 0..count {
                if let Some(msg) = queue.pop_front() {
                    result.push(msg);
                }
            }
            result
        } else {
            Vec::new()
        }
    }

    /// Clear all messages for a component
    pub fn clear(&mut self, component_id: ComponentId) {
        if let Some(queue) = self.queues.get_mut(&component_id) {
            queue.clear();
        }
    }

    /// Get total pending messages across all components
    pub fn total_pending(&self) -> usize {
        self.queues.values().map(|q| q.len()).sum()
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
