use crate::{generate_with_prefix, OnionResult, GeneratorConfig, get_stats};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use anyhow::Result;

/// Message types for worker communication
#[derive(Debug)]
pub enum WorkerMessage {
    Found(OnionResult),
    Stats(u64, u64),
    Shutdown,
}

/// Worker pool for parallel onion generation
pub struct WorkerPool {
    config: Arc<GeneratorConfig>,
    sender: mpsc::Sender<WorkerMessage>,
    receiver: mpsc::Receiver<WorkerMessage>,
    workers: Vec<thread::JoinHandle<()>>,
    stats_thread: Option<thread::JoinHandle<()>>,
}

impl WorkerPool {
    /// Create a new worker pool
    pub fn new(config: GeneratorConfig) -> Self {
        let (sender, receiver) = mpsc::channel();
        
        Self {
            config: Arc::new(config),
            sender,
            receiver,
            workers: Vec::new(),
            stats_thread: None,
        }
    }

    /// Start all workers
    pub fn start(&mut self) -> Result<()> {
        // Start worker threads
        for worker_id in 0..self.config.num_workers {
            let config = Arc::clone(&self.config);
            let sender = self.sender.clone();
            
            let handle = thread::spawn(move || {
                worker_thread(worker_id, config, sender);
            });
            
            self.workers.push(handle);
        }

        // Start statistics thread
        let stats_sender = self.sender.clone();
        let update_interval = self.config.update_interval;
        
        let stats_handle = thread::spawn(move || {
            stats_thread(stats_sender, update_interval);
        });
        
        self.stats_thread = Some(stats_handle);
        
        println!("[@] Started {} worker threads", self.config.num_workers);
        println!("[@] Generating addresses...");
        
        Ok(())
    }

    /// Process messages from workers
    pub fn run(&self) -> Result<()> {
        loop {
            match self.receiver.recv() {
                Ok(WorkerMessage::Found(result)) => {
                    println!("[√] Address generated successfully!");
                    println!("Hostname:                      {}", result.hostname);
                    println!("Public Key (Base64 encoded):   {}", result.public_key);
                    println!("Private Key (Base64 encoded):  {}\n", result.private_key);
                }
                Ok(WorkerMessage::Stats(generated, found)) => {
                    let now = chrono::Local::now();
                    println!("[@] {}: Generated {} addresses, Found {} addresses", 
                             now.format("%H:%M:%S"), generated, found);
                }
                Ok(WorkerMessage::Shutdown) => {
                    break;
                }
                Err(_) => {
                    // Channel closed, exit
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Shutdown all workers
    pub fn shutdown(self) -> Result<()> {
        // Send shutdown signal
        for _ in 0..self.config.num_workers {
            let _ = self.sender.send(WorkerMessage::Shutdown);
        }

        // Wait for all workers to finish
        for handle in self.workers {
            let _ = handle.join();
        }

        // Wait for stats thread
        if let Some(handle) = self.stats_thread {
            let _ = handle.join();
        }

        println!("[!] All workers stopped");
        Ok(())
    }
}

/// Worker thread function
fn worker_thread(
    worker_id: usize,
    config: Arc<GeneratorConfig>,
    sender: mpsc::Sender<WorkerMessage>,
) {
    println!("[+] Worker {} started", worker_id);
    
    loop {
        match generate_with_prefix(&config.prefixes) {
            Ok(result) => {
                if sender.send(WorkerMessage::Found(result)).is_err() {
                    break; // Channel closed
                }
            }
            Err(e) => {
                eprintln!("[!] Worker {} error: {}", worker_id, e);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
    
    println!("[-] Worker {} stopped", worker_id);
}

/// Statistics reporting thread
fn stats_thread(sender: mpsc::Sender<WorkerMessage>, interval_seconds: u64) {
    let interval = Duration::from_secs(interval_seconds);
    
    loop {
        thread::sleep(interval);
        
        let (generated, found) = get_stats();
        if sender.send(WorkerMessage::Stats(generated, found)).is_err() {
            break; // Channel closed
        }
    }
}

/// Simple single-threaded generator for comparison
pub fn run_single_threaded(prefixes: &[String]) -> Result<()> {
    println!("[@] Running in single-threaded mode");
    println!("[@] Generating addresses...");
    
    let start_time = Instant::now();
    let mut last_stats_time = start_time;
    
    loop {
        let result = generate_with_prefix(prefixes)?;
        
        println!("[√] Address generated successfully!");
        println!("Hostname:                      {}", result.hostname);
        println!("Public Key (Base64 encoded):   {}", result.public_key);
        println!("Private Key (Base64 encoded):  {}\n", result.private_key);
        
        // Print stats every 30 seconds
        let now = Instant::now();
        if now.duration_since(last_stats_time) >= Duration::from_secs(30) {
            let (generated, found) = get_stats();
            let current_time = chrono::Local::now();
            println!("[@] {}: Generated {} addresses, Found {} addresses", 
                     current_time.format("%H:%M:%S"), generated, found);
            last_stats_time = now;
        }
    }
}
