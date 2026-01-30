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

use components::{CarSystem, EngineComponent, BrakesComponent};

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
