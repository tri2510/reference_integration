//! Workflow orchestration
//! This demonstrates S-CORE's Orchestrator pattern - sequences of actions

use std::fmt;

/// Workflow step - a single action in a workflow
pub struct WorkflowStep {
    name: String,
    description: String,
    action: Box<dyn Fn(&mut crate::components::system::CarSystem) -> Result<(), String>>,
}

impl WorkflowStep {
    /// Create a new workflow step
    pub fn new(
        name: &str,
        description: &str,
        action: Box<dyn Fn(&mut crate::components::system::CarSystem) -> Result<(), String>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            action,
        }
    }

    /// Execute this step
    pub fn execute(&self, system: &mut crate::components::system::CarSystem) -> Result<(), String> {
        println!("  ▶ Step: {}", self.name);
        (self.action)(system)?;
        println!("  ✅ {}: Complete", self.name);
        Ok(())
    }
}

/// Workflow - a sequence of steps to execute
/// This is like S-CORE's Orchestrator - manages complex procedures
pub struct Workflow {
    name: String,
    description: String,
    steps: Vec<WorkflowStep>,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            steps: Vec::new(),
        }
    }

    /// Add a step to the workflow
    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }

    /// Execute all steps in sequence
    pub fn execute(&self, system: &mut crate::components::system::CarSystem) -> Result<(), String> {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║           📋 Workflow: {:<40} ║", &self.name[..self.name.len().min(40)]);
        println!("║           {:<52}║", self.description);
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("📝 Total steps: {}\n", self.steps.len());

        for (index, step) in self.steps.iter().enumerate() {
            println!("─ Step {}/{} ─────────────────────────────────────────────────", index + 1, self.steps.len());
            step.execute(system)?;
            println!();
        }

        println!("✅ Workflow '{}' completed successfully!\n", self.name);
        Ok(())
    }

    /// Get the number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// Workflow builder - helps construct workflows easily
pub struct WorkflowBuilder {
    workflow: Workflow,
}

impl WorkflowBuilder {
    /// Create a new workflow builder
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            workflow: Workflow::new(name, description),
        }
    }

    /// Add a step to the workflow
    pub fn step(&mut self, name: &str, description: &str,
               action: Box<dyn Fn(&mut crate::components::system::CarSystem) -> Result<(), String>>) -> &mut Self {
        self.workflow.add_step(WorkflowStep::new(name, description, action));
        self
    }

    /// Build the workflow
    pub fn build(self) -> Workflow {
        self.workflow
    }
}

impl fmt::Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Workflow[{}] ({} steps)", self.name, self.steps.len())
    }
}
