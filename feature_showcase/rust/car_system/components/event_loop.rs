//! Event loop for continuous real-time processing
//! This demonstrates S-CORE's event-driven architecture pattern

use std::time::{Duration, Instant};
use std::thread;

/// Event loop configuration
#[derive(Debug, Clone)]
pub struct EventLoopConfig {
    /// Tick rate in milliseconds (how often to process)
    pub tick_rate_ms: u64,
    /// Whether to print timing info
    pub verbose_timing: bool,
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        Self {
            tick_rate_ms: 500,  // 2 Hz by default
            verbose_timing: false,
        }
    }
}

/// Event loop - runs continuously at a fixed tick rate
pub struct EventLoop {
    running: bool,
    config: EventLoopConfig,
    tick_count: u64,
    start_time: Option<Instant>,
}

impl EventLoop {
    /// Create a new event loop
    pub fn new(config: EventLoopConfig) -> Self {
        Self {
            running: false,
            config,
            tick_count: 0,
            start_time: None,
        }
    }

    /// Create with default config
    pub fn default() -> Self {
        Self::new(EventLoopConfig::default())
    }

    /// Check if the event loop is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get current tick count
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Get elapsed time since start
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Start the event loop
    pub fn start(&mut self) {
        self.running = true;
        self.start_time = Some(Instant::now());
        self.tick_count = 0;

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🔄 Event Loop Started");
        println!("   Tick Rate: {} ms ({} Hz)", self.config.tick_rate_ms, 1000 / self.config.tick_rate_ms);
        println!("   Press Ctrl+C to stop");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    /// Stop the event loop
    pub fn stop(&mut self) {
        self.running = false;

        if let Some(elapsed) = self.elapsed() {
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🛑 Event Loop Stopped");
            println!("   Total Ticks: {}", self.tick_count);
            println!("   Total Time: {:.2}s", elapsed.as_secs_f64());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        }
    }

    /// Run one tick of the event loop
    /// Returns the duration the tick took
    pub fn tick<F>(&mut self, mut callback: F) -> Duration
    where
        F: FnMut(u64) -> Result<(), String>,
    {
        let tick_start = Instant::now();

        // Call the callback with current tick number
        if let Err(e) = callback(self.tick_count) {
            eprintln!("❌ Error in tick {}: {}", self.tick_count, e);
        }

        self.tick_count += 1;

        let tick_duration = tick_start.elapsed();

        // Print timing if verbose
        if self.config.verbose_timing {
            println!("   [Timing] Tick {} took: {:.2}ms", self.tick_count - 1, tick_duration.as_secs_f64() * 1000.0);
        }

        tick_duration
    }

    /// Run the event loop with a callback
    /// The callback receives the tick number and should return Result<(), String>
    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(u64) -> Result<(), String>,
    {
        self.start();

        while self.running {
            // Run the tick
            let tick_duration = self.tick(&mut callback);

            // Sleep to maintain tick rate
            let target_duration = Duration::from_millis(self.config.tick_rate_ms);

            if tick_duration < target_duration {
                let sleep_time = target_duration - tick_duration;
                thread::sleep(sleep_time);
            } else {
                // Tick took longer than target - warn
                eprintln!("⚠️  Warning: Tick {} took {:.2}ms (target: {}ms) - can't keep up!",
                    self.tick_count - 1,
                    tick_duration.as_secs_f64() * 1000.0,
                    self.config.tick_rate_ms
                );
            }
        }

        self.stop();
    }

    /// Run for a fixed number of ticks (for testing/demos)
    pub fn run_for<F>(&mut self, num_ticks: u64, mut callback: F)
    where
        F: FnMut(u64) -> Result<(), String>,
    {
        self.start();

        for _ in 0..num_ticks {
            if !self.running {
                break;
            }

            self.tick(&mut callback);

            // Sleep to maintain tick rate
            thread::sleep(Duration::from_millis(self.config.tick_rate_ms));
        }

        self.stop();
    }
}
