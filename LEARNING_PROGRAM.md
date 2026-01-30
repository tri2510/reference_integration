# 🚗 S-CORE Learning Program: Building a Car-Like System

## 📋 Overview

This learning program guides you through understanding Eclipse S-CORE by building a car-like system step-by-step. You'll learn S-CORE patterns, architecture, and workflows by implementing them.

**Repository:** https://github.com/tri2510/reference_integration  
**Base Project:** Eclipse S-CORE (https://eclipse.dev/score/)

---

## 🎯 Learning Goals

By the end of this program, you will understand:
- ✅ S-CORE build system (Bazel)
- ✅ S-CORE error handling patterns
- ✅ S-CORE structured logging
- ✅ Component-based architecture
- ✅ Message passing between components
- ✅ State management
- ✅ Workflow orchestration
- ✅ Safety & fault handling

---

## 📚 Phase 1: Foundation (COMPLETED ✅)

### What You Built:
- [x] S-CORE feature: `hello_score`
- [x] Error handling with `Result<T, E>`
- [x] Command-line interface
- [x] Structured logging with levels
- [x] Component-based architecture

### Files Created:
```
feature_showcase/rust/hello_score/
├── main.rs          (S-CORE style component with logging)
└── BUILD            (Bazel build rule)
```

### Commands to Build & Run:
```bash
cd /home/htr1hc/01_PJNE/06_score

# Build
bazel-8.3.0 build //feature_showcase/rust:hello_score_example

# Run (default)
bazel-8.3.0 run //feature_showcase/rust:hello_score_example

# Run with custom settings
bazel-8.3.0 run //feature_showcase/rust:hello_score_example -- --name "TestComponent" --log-level DEBUG

# Show help
bazel-8.3.0 run //feature_showcase/rust:hello_score_example -- --help
```

### S-CORE Patterns Learned:
| Pattern | Description | Real S-CORE Usage |
|---------|-------------|-------------------|
| **Result<T, E>** | Error handling | All S-CORE functions return results |
| **Config struct** | Configuration | S-CORE components use config structs |
| **Structured logging** | Component logging | S-CORE uses structured logging |
| **Log levels** | ERROR→WARN→INFO→DEBUG→TRACE | S-CORE has 5 log levels |
| **Component** | Self-contained unit | S-CORE is component-based |

---

## 📚 Phase 2: Multi-Component Architecture (NEXT)

### Goal:
Create multiple car components that work together:
- Engine Component (manages engine state)
- Brakes Component (manages braking)
- Steering Component (manages steering)
- Dashboard Component (displays status)

### Implementation Steps:

#### Step 1: Define Component Trait
```rust
/// All car components implement this trait
trait CarComponent {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> Result<(), String>;
    fn process(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn get_state(&self) -> ComponentState;
}
```

#### Step 2: Create Engine Component
```rust
struct EngineComponent {
    state: EngineState,
    rpm: u32,
    temperature: f32,
    logger: ScoreLogger,
}

enum EngineState {
    Off,
    Starting,
    Running,
    Stopping,
}
```

#### Step 3: Create Brakes Component
```rust
struct BrakesComponent {
    applied: bool,
    pressure: u8,  // 0-100%
    logger: ScoreLogger,
}
```

#### Step 4: Create Steering Component
```rust
struct SteeringComponent {
    angle: i16,  // -90 to +90 degrees
    logger: ScoreLogger,
}
```

#### Step 5: Create Dashboard Component
```rust
struct DashboardComponent {
    speed: u8,
    engine_rpm: u32,
    fuel_level: u8,
    warnings: Vec<String>,
}
```

#### Step 6: Create Component Manager
```rust
struct CarSystem {
    components: Vec<Box<dyn CarComponent>>,
    logger: ScoreLogger,
}
```

**File Structure:**
```
feature_showcase/rust/car_system/
├── main.rs              (Entry point)
├── components/
│   ├── mod.rs          (Component definitions)
│   ├── engine.rs       (Engine component)
│   ├── brakes.rs       (Brakes component)
│   ├── steering.rs     (Steering component)
│   └── dashboard.rs    (Dashboard component)
└── BUILD               (Build rules)
```

---

## 📚 Phase 3: Communication Between Components

### Goal:
Components send messages to each other (like S-CORE Communication module)

### Implementation:
```rust
/// Message types
enum CarMessage {
    EngineStart,
    EngineStop,
    Accelerate(u8),
    Brake(u8),
    Steer(i16),
    RequestState,
}

/// Message bus (central communication hub)
struct MessageBus {
    queues: HashMap<ComponentId, Vec<CarMessage>>,
}

/// Components can publish and subscribe
impl MessageBus {
    fn publish(&mut self, from: ComponentId, msg: CarMessage);
    fn subscribe(&mut self, component: ComponentId);
    fn receive(&mut self, component: ComponentId) -> Vec<CarMessage>;
}
```

**This demonstrates S-CORE's communication pattern!**

---

## 📚 Phase 4: State Management

### Goal:
Components have state and transition between states

### Implementation:
```rust
/// Generic state machine
trait StateMachine {
    type State;
    fn current_state(&self) -> Self::State;
    fn transition(&mut self, new_state: Self::State) -> Result<(), String>;
}

/// Engine state machine
impl StateMachine for EngineComponent {
    type State = EngineState;
    
    fn transition(&mut self, new_state: EngineState) -> Result<(), String> {
        match (&self.state, &new_state) {
            (EngineState::Off, EngineState::Starting) => {
                self.logger.info("Starting engine...");
                self.state = new_state;
                Ok(())
            }
            (EngineState::Starting, EngineState::Running) => {
                self.logger.info("Engine running!");
                self.state = new_state;
                Ok(())
            }
            // ... more transitions
            _ => Err(format!("Invalid transition: {:?} -> {:?}", self.state, new_state)),
        }
    }
}
```

**This demonstrates S-CORE's workflow patterns!**

---

## 📚 Phase 5: Event Loop (Real-time Processing)

### Goal:
Continuous processing like a real car system

### Implementation:
```rust
struct EventLoop {
    running: bool,
    tick_rate_ms: u64,
    components: Vec<Box<dyn CarComponent>>,
}

impl EventLoop {
    fn run(&mut self) {
        while self.running {
            // Process all components
            for component in &mut self.components {
                component.process().unwrap();
            }
            
            // Sleep for tick rate
            std::thread::sleep(Duration::from_millis(self.tick_rate_ms));
        }
    }
}
```

---

## 📚 Phase 6: Safety & Fault Handling

### Goal:
Implement safety checks (like ISO 26262)

### Implementation:
```rust
/// Safety monitor
struct SafetyMonitor {
    max_speed: u8,
    max_temperature: f32,
    max_rpm: u32,
}

impl SafetyMonitor {
    fn check(&self, state: &CarSystemState) -> Vec<SafetyWarning> {
        let mut warnings = Vec::new();
        
        if state.speed > self.max_speed {
            warnings.push(SafetyWarning::SpeedExceeded);
        }
        
        if state.engine_temperature > self.max_temperature {
            warnings.push(SafetyWarning::Overheating);
        }
        
        warnings
    }
}
```

---

## 📚 Phase 7: Workflow Orchestration

### Goal:
Execute workflows (like S-CORE Orchestrator)

### Implementation:
```rust
/// Workflow step
struct WorkflowStep {
    name: String,
    action: Box<dyn Fn(&mut CarSystem) -> Result<(), String>>,
}

/// Workflow (sequence of steps)
struct Workflow {
    steps: Vec<WorkflowStep>,
}

impl Workflow {
    fn execute(&self, system: &mut CarSystem) -> Result<(), String> {
        for step in &self.steps {
            (step.action)(system)?;
        }
        Ok(())
    }
}

/// Example: Start car workflow
fn create_start_workflow() -> Workflow {
    Workflow {
        steps: vec![
            WorkflowStep {
                name: "Check brakes".to_string(),
                action: Box::new(|system| system.check_brakes()),
            },
            WorkflowStep {
                name: "Start engine".to_string(),
                action: Box::new(|system| system.start_engine()),
            },
            WorkflowStep {
                name: "Enable dashboard".to_string(),
                action: Box::new(|system| system.enable_dashboard()),
            },
        ],
    }
}
```

**This demonstrates S-CORE's Orchestrator pattern!**

---

## 📚 Phase 8: Complete Car Simulation

### Goal:
Full car-like system with all features

### Features:
- ✅ Multiple components (Engine, Brakes, Steering, Dashboard)
- ✅ Message passing between components
- ✅ State management with state machines
- ✅ Event loop for continuous processing
- ✅ Safety checks and fault handling
- ✅ Workflow orchestration
- ✅ Interactive CLI (accelerate, brake, steer)
- ✅ Real-time dashboard display

---

## 🛠️ Development Workflow

### Creating a New Feature:
```bash
cd /home/htr1hc/01_PJNE/06_score

# 1. Create feature folder
mkdir -p feature_showcase/rust/my_feature

# 2. Write code
nano feature_showcase/rust/my_feature/main.rs

# 3. Add BUILD rule
nano feature_showcase/rust/BUILD

# 4. Build
bazel-8.3.0 build //feature_showcase/rust:my_feature_example

# 5. Run
bazel-8.3.0 run //feature_showcase/rust:my_feature_example

# 6. Commit changes
git add .
git commit -m "Add my new feature"
git push
```

---

## 📖 Key Concepts Map

| S-CORE Concept | Your Implementation | Location |
|----------------|---------------------|----------|
| **ScoreError** | `ScoreError` enum | Phase 1 |
| **ScoreResult** | `ScoreResult<T>` type | Phase 1 |
| **Structured Logging** | `ScoreLogger` | Phase 1 |
| **Component** | `CarComponent` trait | Phase 2 |
| **Communication** | `MessageBus` | Phase 3 |
| **State Machine** | `StateMachine` trait | Phase 4 |
| **Event Loop** | `EventLoop` | Phase 5 |
| **Safety Monitor** | `SafetyMonitor` | Phase 6 |
| **Workflow** | `Workflow` struct | Phase 7 |
| **Orchestrator** | `CarSystem` manager | Phase 8 |

---

## 🎯 Milestones

- [x] **Milestone 1**: Build and run first S-CORE feature
- [ ] **Milestone 2**: Create multi-component system (4 components)
- [ ] **Milestone 3**: Add message passing between components
- [ ] **Milestone 4**: Implement state machines
- [ ] **Milestone 5**: Add event loop
- [ ] **Milestone 6**: Implement safety checks
- [ ] **Milestone 7**: Create workflow orchestration
- [ ] **Milestone 8**: Complete car simulation

---

## 📚 Additional Resources

- **Eclipse S-CORE Handbook**: https://eclipse-score.github.io/score/main/handbook/index.html
- **S-CORE Architecture**: https://eclipse.dev/score/docs.html
- **GitHub Organization**: https://github.com/eclipse-score
- **Your Fork**: https://github.com/tri2510/reference_integration

---

## 🚀 Next Steps

**Ready to start Phase 2?**

Run: `cat /home/htr1hc/01_PJNE/06_score/PHASE2.md`

---

*Generated: 2025-01-30*  
*Author: Claude & tri2510*  
*License: Apache-2.0*
