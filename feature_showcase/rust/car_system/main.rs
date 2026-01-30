//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// SPDX-License-Identifier: Apache-2.0
//

//! S-CORE Car System - Phase 3: Component Communication
//!
//! This example demonstrates S-CORE patterns:
//! - Component-based architecture
//! - Component lifecycle management
//! - State management
//! - Message-based communication (NEW!)
//! - Orchestration of multiple components

mod components;

use components::*;

use std::thread;
use std::time::Duration;

/// Car system - orchestrates all components
/// This demonstrates S-CORE's orchestration pattern
struct CarSystem {
    engine: EngineComponent,
    brakes: BrakesComponent,
    steering: SteeringComponent,
    dashboard: DashboardComponent,
    message_bus: MessageBus,  // Phase 3: Communication hub
}

impl CarSystem {
    /// Create a new car system with all components
    fn new() -> Self {
        let mut message_bus = MessageBus::new();

        // Register all components with the message bus
        message_bus.register_component(ComponentId::Engine);
        message_bus.register_component(ComponentId::Brakes);
        message_bus.register_component(ComponentId::Steering);
        message_bus.register_component(ComponentId::Dashboard);

        // Dashboard subscribes to all messages
        message_bus.subscribe_all(ComponentId::Dashboard);

        Self {
            engine: EngineComponent::new(),
            brakes: BrakesComponent::new(),
            steering: SteeringComponent::new(),
            dashboard: DashboardComponent::new(),
            message_bus,
        }
    }

    /// Initialize all components
    /// This follows S-CORE's initialization pattern
    fn initialize(&mut self) -> Result<(), String> {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║          🚗 S-CORE Car System - Phase 3                    ║");
        println!("║    Multi-Component Architecture + Communication           ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("🔧 Initializing message bus...");
        println!("✅ Message bus ready\n");

        println!("🔧 Initializing all components...\n");

        // Initialize each component
        self.engine.initialize()?;
        println!();
        self.brakes.initialize()?;
        println!();
        self.steering.initialize()?;
        println!();
        self.dashboard.initialize()?;

        println!("\n✅ All components initialized successfully!\n");
        Ok(())
    }

    /// Start the car (start engine)
    fn start(&mut self) -> Result<(), String> {
        println!("🔑 Starting the car...\n");
        self.engine.start()?;

        // Set initial dashboard state
        self.dashboard.set_fuel_level(85);

        println!("\n✅ Car is ready to drive!\n");
        Ok(())
    }

    /// Run a driving scenario
    fn run_scenario(&mut self) -> Result<(), String> {
        println!("🚗 Starting driving scenario...\n");
        println!("{}", "━".repeat(60));

        // Scenario: Accelerate from 0 to 100 km/h
        for i in 1..=10 {
            self.process_cycle(i * 10)?;
            thread::sleep(Duration::from_millis(500));
        }

        // Scenario: Turn while braking
        println!("\n🔄 Making a turn...");
        self.steering.turn(30)?;
        for i in 1..=3 {
            self.process_cycle(100)?;
            thread::sleep(Duration::from_millis(400));
        }

        println!("\n🛞 Applying brakes...");
        self.brakes.apply(60)?;
        for i in (0..20).rev() {
            self.process_cycle(i * 5)?;
            thread::sleep(Duration::from_millis(300));
        }

        // Center steering
        println!("\n🔄 Centering steering...");
        self.steering.center();

        println!("\n✅ Scenario complete!\n");
        Ok(())
    }

    /// Process one cycle - update all components and exchange messages
    fn process_cycle(&mut self, speed: u8) -> Result<(), String> {
        // Update all components first (so state changes happen)
        self.engine.process()?;
        self.brakes.process()?;
        self.steering.process()?;

        // Phase 3: Collect messages from all components AFTER they've processed
        let mut engine_msgs = self.engine.get_messages();
        let mut brakes_msgs = self.brakes.get_messages();
        let mut steering_msgs = self.steering.get_messages();

        // Publish messages to the bus
        for msg in engine_msgs.drain(..) {
            self.message_bus.publish(ComponentId::Engine, msg);
        }
        for msg in brakes_msgs.drain(..) {
            self.message_bus.publish(ComponentId::Brakes, msg);
        }
        for msg in steering_msgs.drain(..) {
            self.message_bus.publish(ComponentId::Steering, msg);
        }

        // Dashboard receives all messages
        let dashboard_msgs = self.message_bus.receive_all(ComponentId::Dashboard);
        if !dashboard_msgs.is_empty() {
            self.dashboard.process_messages(dashboard_msgs);
        }

        // Update dashboard with current component states
        self.dashboard.set_speed(speed);
        self.dashboard.update_odometer(speed as f32 / 10.0);
        self.dashboard.process()?;

        // Display dashboard with component states
        println!();
        self.dashboard.display(
            self.engine.get_rpm(),
            self.engine.get_temperature(),
            self.brakes.get_pressure(),
            self.steering.get_angle(),
        );

        Ok(())
    }

    /// Shutdown the car
    fn shutdown(&mut self) -> Result<(), String> {
        println!("🛑 Shutting down the car...\n");
        self.engine.stop()?;
        println!("\n✅ Car shut down complete!");
        println!("{}", "━".repeat(60));
        Ok(())
    }
}

/// Main entry point
fn main() -> Result<(), String> {
    let mut car = CarSystem::new();

    // Follow the S-CORE lifecycle pattern:
    // 1. Initialize
    car.initialize()?;

    // 2. Start
    car.start()?;

    // 3. Run (process cycles)
    car.run_scenario()?;

    // 4. Shutdown
    car.shutdown()?;

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           ✅ Phase 3 Complete!                               ║");
    println!("║                                                                ║");
    println!("║  You've learned:                                              ║");
    println!("║  ✓ Component-based architecture                               ║");
    println!("║  ✓ Component lifecycle management                             ║");
    println!("║  ✓ State management                                           ║");
    println!("║  ✓ Multi-component orchestration                              ║");
    println!("║  ✓ Message-based communication (NEW!)                         ║");
    println!("║  ✓ Publish-subscribe pattern (NEW!)                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    Ok(())
}
