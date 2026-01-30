//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// SPDX-License-Identifier: Apache-2.0
//

use std::env;

/// S-CORE style log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN ",
            LogLevel::Info => "INFO ",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

/// S-CORE style structured log entry
struct LogEntry {
    level: LogLevel,
    component: String,
    message: String,
    timestamp: u64,
}

impl LogEntry {
    fn new(level: LogLevel, component: &str, message: &str) -> Self {
        Self {
            level,
            component: component.to_string(),
            message: message.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
    
    fn format(&self) -> String {
        format!("[{:?}] {}: {}", self.level, self.component, self.message)
    }
}

/// S-CORE style logger
struct ScoreLogger {
    min_level: LogLevel,
    component: String,
}

impl ScoreLogger {
    fn new(component: &str, min_level: LogLevel) -> Self {
        Self {
            min_level,
            component: component.to_string(),
        }
    }
    
    fn log(&self, level: LogLevel, message: &str) {
        if level <= self.min_level {
            let entry = LogEntry::new(level, &self.component, message);
            println!("{}", entry.format());
        }
    }
    
    fn error(&self, message: &str) { self.log(LogLevel::Error, message); }
    fn warn(&self, message: &str) { self.log(LogLevel::Warn, message); }
    fn info(&self, message: &str) { self.log(LogLevel::Info, message); }
    fn debug(&self, message: &str) { self.log(LogLevel::Debug, message); }
    fn trace(&self, message: &str) { self.log(LogLevel::Trace, message); }
}

/// S-CORE style configuration
struct ScoreConfig {
    name: String,
    log_level: LogLevel,
}

fn parse_log_level(s: &str) -> Result<LogLevel, String> {
    match s.to_uppercase().as_str() {
        "ERROR" => Ok(LogLevel::Error),
        "WARN" => Ok(LogLevel::Warn),
        "INFO" => Ok(LogLevel::Info),
        "DEBUG" => Ok(LogLevel::Debug),
        "TRACE" => Ok(LogLevel::Trace),
        _ => Err(format!("Invalid log level: {}", s)),
    }
}

impl ScoreConfig {
    fn from_args() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        
        let mut name = "S-CORE".to_string();
        let mut log_level = LogLevel::Info;
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--name" | "-n" => {
                    if i + 1 < args.len() {
                        name = args[i + 1].clone();
                        i += 2;
                    } else {
                        return Err("--name requires a value".to_string());
                    }
                }
                "--log-level" | "-l" => {
                    if i + 1 < args.len() {
                        log_level = parse_log_level(&args[i + 1])?;
                        i += 2;
                    } else {
                        return Err("--log-level requires a value".to_string());
                    }
                }
                "--help" | "-h" => {
                    return Err("HELP".to_string());
                }
                _ => {
                    return Err(format!("Unknown argument: {}", args[i]));
                }
            }
        }
        
        Ok(Self { name, log_level })
    }
    
    fn print_help() {
        println!("🎯 S-CORE Feature - Structured Logging");
        println!();
        println!("USAGE:");
        println!("  hello_score_example [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("  -n, --name <NAME>         Set component name [default: S-CORE]");
        println!("  -l, --log-level <LEVEL>   Set log level [ERROR|WARN|INFO|DEBUG|TRACE]");
        println!("  -h, --help                Print this help");
        println!();
        println!("EXAMPLES:");
        println!("  hello_score_example");
        println!("  hello_score_example --name MyComponent --log-level DEBUG");
    }
}

/// Simulated S-CORE component
struct ScoreComponent {
    logger: ScoreLogger,
    counter: u64,
}

impl ScoreComponent {
    fn new(name: &str, log_level: LogLevel) -> Self {
        let logger = ScoreLogger::new(name, log_level);
        logger.info(&format!("Component '{}' initialized", name));
        
        Self {
            logger,
            counter: 0,
        }
    }
    
    fn process(&mut self) {
        self.logger.trace("Starting process");
        
        self.counter += 1;
        self.logger.debug(&format!("Counter incremented to {}", self.counter));
        
        if self.counter % 10 == 0 {
            self.logger.warn(&format!("Counter reached milestone: {}", self.counter));
        }
        
        if self.counter % 100 == 0 {
            self.logger.info(&format!("Counter reached century: {}", self.counter));
        }
        
        self.logger.trace("Process complete");
    }
}

/// Main entry point
fn main() -> Result<(), String> {
    let config = match ScoreConfig::from_args() {
        Ok(cfg) => cfg,
        Err(e) if e == "HELP" => {
            ScoreConfig::print_help();
            return Ok(());
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            ScoreConfig::print_help();
            return Err(e);
        }
    };
    
    let mut component = ScoreComponent::new(&config.name, config.log_level);
    
    component.logger.info("Starting processing loop");
    
    for i in 0..20 {
        component.process();
        
        if config.log_level >= LogLevel::Debug {
            component.logger.trace(&format!("Iteration {} complete", i + 1));
        }
    }
    
    component.logger.info("Processing complete");
    component.logger.debug(&format!("Final counter value: {}", component.counter));
    
    Ok(())
}
