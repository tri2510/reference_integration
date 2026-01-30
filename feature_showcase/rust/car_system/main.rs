//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// SPDX-License-Identifier: Apache-2.0
//

//! S-CORE Car System - Phase 7: Workflow Orchestration
//!
//! This example demonstrates S-CORE patterns:
//! - Component-based architecture
//! - Component lifecycle management
//! - State management with state machines
//! - Valid state transitions
//! - Message-based communication
//! - Event loop for continuous processing
//! - Safety monitoring and fault handling
//! - Workflow orchestration (NEW!)

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
    message_bus: MessageBus,    // Phase 3: Communication hub
    safety: SafetyMonitor,        // Phase 6: Safety monitoring
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
            safety: SafetyMonitor::new(), // Phase 6: Safety monitor
        }
    }

    /// Initialize all components
    /// This follows S-CORE's initialization pattern
    fn initialize(&mut self) -> Result<(), String> {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║          🚗 S-CORE Car System - Phase 7                    ║");
        println!("║  Multi-Component + Comm + State Machine + Loop + Safety + Workflows ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("🔧 Initializing message bus...");
        println!("✅ Message bus ready");

        println!("🔧 Initializing safety monitor...");
        println!("   Limits: Speed={}km/h, Temp={}°C, RPM={}",
                 self.safety.max_speed, self.safety.max_temperature, self.safety.max_rpm);
        println!("✅ Safety monitor ready");

        println!("🔧 Initializing workflow orchestrator...");
        println!("✅ Workflow orchestrator ready\n");

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

        // Phase 4: Demonstrate state machine validation
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📚 Phase 4: State Machine Validation Demo");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        println!("\n✅ Valid transitions (what just happened):");
        println!("   OFF → STARTING → RUNNING");

        println!("\n🧪 Testing invalid transition (try to start already-running engine):");
        match self.engine.start() {
            Ok(_) => println!("   ❌ Oops - should have failed!"),
            Err(e) => println!("   ✅ Correctly rejected: {}", e),
        }

        println!("\n📊 Current engine state: {}\n", self.engine.get_engine_state());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

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

    /// Run event loop for continuous processing (Phase 5 & 6)
    pub fn run_event_loop(&mut self, num_ticks: u64) -> Result<(), String> {
        let config = EventLoopConfig {
            tick_rate_ms: 500,  // 2 Hz
            verbose_timing: false,
        };

        let mut event_loop = EventLoop::new(config);
        let mut speed = 0u8;
        let mut accelerating = true;

        // Phase 6: Show safety demo at start
        if num_ticks > 10 {
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📚 Phase 6: Safety Monitor Demo");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            // Trigger safety warnings by exceeding limits
            println!("\n🧪 Triggering safety warnings for demo...\n");

            // Speed warning
            let warnings = self.safety.check(130, 85.0, 5000, 50, 0, true);
            for warning in &warnings {
                println!("   {}", warning);
            }

            println!("\n✅ Safety monitor active - will warn during operation\n");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        }

        // Event loop callback
        event_loop.run_for(num_ticks, |tick_num| {
            // Simulate speed oscillation (0-130 km/h - exceed limit for demo)
            if tick_num % 25 == 0 {
                if accelerating {
                    if speed >= 130 {  // Exceed safety limit
                        accelerating = false;
                    }
                } else {
                    if speed == 0 {
                        accelerating = true;
                    }
                }
            }

            if accelerating && speed < 130 {
                speed += 5;
            } else if !accelerating && speed > 0 {
                speed -= 5;
            }

            // Apply brakes occasionally
            if tick_num % 30 == 0 && tick_num > 0 {
                self.brakes.apply(50)?;
            } else if tick_num % 30 == 10 {
                self.brakes.release();
            }

            // Turn occasionally
            if tick_num % 25 == 15 {
                self.steering.turn(30)?;
            } else if tick_num % 25 == 20 {
                self.steering.center();
            }

            // Process one cycle
            self.process_cycle(speed)?;

            // Phase 6: Run safety checks every 5 ticks
            if tick_num % 5 == 0 {
                let warnings = self.safety.check(
                    speed,
                    self.engine.get_temperature(),
                    self.engine.get_rpm(),
                    self.dashboard.get_fuel_level(),
                    self.brakes.get_pressure(),
                    self.engine.is_running()
                );

                if !warnings.is_empty() {
                    println!("\n⚠️  SAFETY CHECK:");
                    for warning in &warnings {
                        println!("   {}", warning);
                    }

                    // Check if system is still safe
                    if !self.safety.is_safe(&warnings) {
                        println!("   🔴 CRITICAL SAFETY ISSUE - Consider stopping!");
                    }
                    println!();
                }
            }

            Ok(())
        });

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

    /// Create a "Start Car" workflow
    pub fn create_start_workflow() -> Workflow {
        WorkflowBuilder::new(
            "Start Car",
            "Sequence to start the car and prepare for driving"
        )
        .step(
            "Start Engine",
            "Initialize the engine",
            Box::new(|system| {
                println!("🔑 Turning key to start engine...");
                system.engine.start()?;
                Ok(())
            }),
        )
        .step(
            "Initialize Dashboard",
            "Set initial dashboard values",
            Box::new(|system| {
                println!("📊 Setting up dashboard...");
                system.dashboard.set_fuel_level(85);
                Ok(())
            }),
        )
        .step(
            "Ready Announcement",
            "Announce car is ready",
            Box::new(|_system| {
                println!("\n✅ Car is ready to drive!\n");
                Ok(())
            }),
        )
        .build()
    }

    /// Create a "Shutdown Car" workflow
    pub fn create_shutdown_workflow() -> Workflow {
        WorkflowBuilder::new(
            "Shutdown Car",
            "Sequence to safely shutdown the car"
        )
        .step(
            "Release Brakes",
            "Ensure brakes are released",
            Box::new(|system| {
                println!("🛞 Releasing brakes...");
                system.brakes.release();
                Ok(())
            }),
        )
        .step(
            "Center Steering",
            "Return steering to center",
            Box::new(|system| {
                println!("🔄 Centering steering...");
                system.steering.center();
                Ok(())
            }),
        )
        .step(
            "Stop Engine",
            "Turn off the engine",
            Box::new(|system| {
                println!("🔑 Turning off engine...");
                system.engine.stop()?;
                Ok(())
            }),
        )
        .build()
    }

    /// Create an "Emergency Stop" workflow
    pub fn create_emergency_stop_workflow() -> Workflow {
        WorkflowBuilder::new(
            "Emergency Stop",
            "Immediate emergency stop sequence"
        )
        .step(
            "Max Brakes",
            "Apply maximum brake pressure",
            Box::new(|system| {
                println!("🚨 APPLYING MAXIMUM BRAKES!");
                system.brakes.apply(100)?;
                Ok(())
            }),
        )
        .step(
            "Stop Engine",
            "Immediately stop engine",
            Box::new(|system| {
                println!("🚨 STOPPING ENGINE!");
                system.engine.stop()?;
                Ok(())
            }),
        )
        .step(
            "Hazard Warning",
            "Display emergency status",
            Box::new(|_system| {
                println!("\n🚨 EMERGENCY STOP COMPLETE! 🚨");
                println!("   Vehicle safely stopped\n");
                Ok(())
            }),
        )
        .build()
    }
}

/// Main entry point
fn main() -> Result<(), String> {
    let mut car = CarSystem::new();

    // Phase 7: Use workflows instead of manual steps
    println!("\n{}\n", "━".repeat(60));
    println!("🎭 PHASE 7: Workflow Orchestration Demonstration");
    println!("{}\n", "━".repeat(60));

    // 1. Initialize components
    car.initialize()?;

    // 2. Execute Start Car workflow
    let start_workflow = CarSystem::create_start_workflow();
    start_workflow.execute(&mut car)?;

    // 3. Run event loop
    car.run_event_loop(30)?;

    // 4. Execute Shutdown workflow
    println!("\n{}", "━".repeat(60));
    println!("🎭 Executing Shutdown Workflow...");
    println!("{}\n", "━".repeat(60));

    let shutdown_workflow = CarSystem::create_shutdown_workflow();
    shutdown_workflow.execute(&mut car)?;

    // 5. Demo: Emergency Stop workflow
    println!("\n{}", "━".repeat(60));
    println!("🚨 EMERGENCY STOP WORKFLOW (Demo)");
    println!("{}\n", "━".repeat(60));

    let emergency_workflow = CarSystem::create_emergency_stop_workflow();

    // Re-initialize for demo
    car.engine = EngineComponent::new();
    car.brakes = BrakesComponent::new();

    emergency_workflow.execute(&mut car)?;

    car.shutdown()?;

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           ✅ Phase 7 Complete!                               ║");
    println!("║                                                                ║");
    println!("║  You've learned:                                              ║");
    println!("║  ✓ Component-based architecture                               ║");
    println!("║  ✓ Component lifecycle management                             ║");
    println!("║  ✓ State management                                           ║");
    println!("║  ✓ Multi-component orchestration                              ║");
    println!("║  ✓ Message-based communication                                ║");
    println!("║  ✓ Publish-subscribe pattern                                  ║");
    println!("║  ✓ State machine pattern                                      ║");
    println!("║  ✓ Valid state transitions                                    ║");
    println!("║  ✓ Event loop for continuous processing                        ║");
    println!("║  ✓ Real-time tick-based processing                             ║");
    println!("║  ✓ Safety monitoring                                         ║");
    println!("║  ✓ Fault handling with severity levels                         ║");
    println!("║  ✓ ISO 26262 style safety checks                             ║");
    println!("║  ✓ Workflow orchestration (NEW!)                              ║");
    println!("║  ✓ Sequential action execution (NEW!)                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    Ok(())
}
