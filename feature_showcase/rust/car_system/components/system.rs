//! Car system orchestration
//! This module contains the main CarSystem struct that coordinates all components

use std::thread;
use std::time::Duration;

use crate::components::*;

/// Car system - orchestrates all components
/// This demonstrates S-CORE's orchestration pattern
pub struct CarSystem {
    pub engine: EngineComponent,
    pub brakes: BrakesComponent,
    pub steering: SteeringComponent,
    pub dashboard: DashboardComponent,
    pub message_bus: MessageBus,
    pub safety: SafetyMonitor,
}

impl CarSystem {
    /// Create a new car system with all components
    pub fn new() -> Self {
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
            safety: SafetyMonitor::new(),
        }
    }

    /// Initialize all components
    pub fn initialize(&mut self) -> Result<(), String> {
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

    /// Start the car
    pub fn start(&mut self) -> Result<(), String> {
        println!("🔑 Starting the car...\n");
        self.engine.start()?;
        self.dashboard.set_fuel_level(85);
        println!("\n✅ Car is ready to drive!\n");

        // Demonstrate state machine validation
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

    /// Run event loop for continuous processing
    pub fn run_event_loop(&mut self, num_ticks: u64) -> Result<(), String> {
        let config = EventLoopConfig {
            tick_rate_ms: 500,
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

            println!("\n🧪 Triggering safety warnings for demo...\n");

            let warnings = self.safety.check(130, 85.0, 5000, 50, 0, true);
            for warning in &warnings {
                println!("   {}", warning);
            }

            println!("\n✅ Safety monitor active - will warn during operation\n");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        }

        event_loop.run_for(num_ticks, |tick_num| {
            // Simulate speed oscillation
            if tick_num % 25 == 0 {
                if accelerating {
                    if speed >= 130 {
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

            self.process_cycle(speed)?;

            // Safety checks every 5 ticks
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

    /// Process one cycle
    pub fn process_cycle(&mut self, speed: u8) -> Result<(), String> {
        // Update all components
        self.engine.process()?;
        self.brakes.process()?;
        self.steering.process()?;

        // Collect messages from components
        let mut engine_msgs = self.engine.get_messages();
        let mut brakes_msgs = self.brakes.get_messages();
        let mut steering_msgs = self.steering.get_messages();

        // Publish to bus
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

        // Update dashboard
        self.dashboard.set_speed(speed);
        self.dashboard.update_odometer(speed as f32 / 10.0);
        self.dashboard.process()?;

        // Display
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
    pub fn shutdown(&mut self) -> Result<(), String> {
        println!("🛑 Shutting down the car...\n");
        self.engine.stop()?;
        println!("\n✅ Car shut down complete!");
        println!("{}", "━".repeat(60));
        Ok(())
    }

    /// Create a "Start Car" workflow
    pub fn create_start_workflow() -> super::Workflow {
        let mut builder = crate::components::WorkflowBuilder::new(
            "Start Car",
            "Sequence to start the car and prepare for driving"
        );
        builder.step(
            "Start Engine",
            "Initialize the engine",
            Box::new(|system| {
                println!("🔑 Turning key to start engine...");
                system.engine.start()?;
                Ok(())
            }),
        );
        builder.step(
            "Initialize Dashboard",
            "Set initial dashboard values",
            Box::new(|system| {
                println!("📊 Setting up dashboard...");
                system.dashboard.set_fuel_level(85);
                Ok(())
            }),
        );
        builder.step(
            "Ready Announcement",
            "Announce car is ready",
            Box::new(|_system| {
                println!("\n✅ Car is ready to drive!\n");
                Ok(())
            }),
        );
        builder.build()
    }

    /// Create a "Shutdown Car" workflow
    pub fn create_shutdown_workflow() -> super::Workflow {
        let mut builder = crate::components::WorkflowBuilder::new(
            "Shutdown Car",
            "Sequence to safely shutdown the car"
        );
        builder.step(
            "Release Brakes",
            "Ensure brakes are released",
            Box::new(|system| {
                println!("🛞 Releasing brakes...");
                system.brakes.release();
                Ok(())
            }),
        );
        builder.step(
            "Center Steering",
            "Return steering to center",
            Box::new(|system| {
                println!("🔄 Centering steering...");
                system.steering.center();
                Ok(())
            }),
        );
        builder.step(
            "Stop Engine",
            "Turn off the engine",
            Box::new(|system| {
                println!("🔑 Turning off engine...");
                system.engine.stop()?;
                Ok(())
            }),
        );
        builder.build()
    }

    /// Create an "Emergency Stop" workflow
    pub fn create_emergency_stop_workflow() -> super::Workflow {
        let mut builder = crate::components::WorkflowBuilder::new(
            "Emergency Stop",
            "Immediate emergency stop sequence"
        );
        builder.step(
            "Max Brakes",
            "Apply maximum brake pressure",
            Box::new(|system| {
                println!("🚨 APPLYING MAXIMUM BRAKES!");
                system.brakes.apply(100)?;
                Ok(())
            }),
        );
        builder.step(
            "Stop Engine",
            "Immediately stop engine",
            Box::new(|system| {
                println!("🚨 STOPPING ENGINE!");
                system.engine.stop()?;
                Ok(())
            }),
        );
        builder.step(
            "Hazard Warning",
            "Display emergency status",
            Box::new(|_system| {
                println!("\n🚨 EMERGENCY STOP COMPLETE! 🚨");
                println!("   Vehicle safely stopped\n");
                Ok(())
            }),
        );
        builder.build()
    }
}
