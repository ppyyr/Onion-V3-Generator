use clap::{Arg, Command};
use onion_generator::{GeneratorConfig, WorkerPool, run_single_threaded};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use anyhow::Result;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() -> Result<()> {
    let matches = Command::new("onion-generator")
        .version("0.1.0")
        .author("ppyyr <ppyyr@live.jp>")
        .about("A fast Tor .onion V3 address generator with multi-process support")
        .arg(
            Arg::new("prefixes")
                .help("List of prefixes for the hostname")
                .required(true)
                .num_args(1..)
                .value_name("PREFIX")
        )
        .arg(
            Arg::new("workers")
                .short('w')
                .long("workers")
                .help("Number of worker threads (default: number of CPU cores)")
                .value_name("NUM")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("single-threaded")
                .short('s')
                .long("single-threaded")
                .help("Run in single-threaded mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("update-interval")
                .short('u')
                .long("update-interval")
                .help("Statistics update interval in seconds (default: 30)")
                .value_name("SECONDS")
                .value_parser(clap::value_parser!(u64))
                .default_value("30")
        )
        .get_matches();

    // Parse prefixes
    let prefixes: Vec<String> = matches
        .get_many::<String>("prefixes")
        .unwrap()
        .map(|s| s.trim().to_lowercase())
        .collect();

    if prefixes.is_empty() {
        eprintln!("[!] Error: At least one prefix must be provided.");
        std::process::exit(1);
    }

    println!("[@] Onion V3 Address Generator");
    println!("[@] Searching for prefixes: {:?}", prefixes);

    // Setup signal handler
    setup_signal_handler();

    // Check if single-threaded mode is requested
    if matches.get_flag("single-threaded") {
        return run_single_threaded_with_input(&prefixes);
    }

    // Setup multi-threaded configuration
    let mut config = GeneratorConfig::new(prefixes);
    
    if let Some(workers) = matches.get_one::<usize>("workers") {
        config = config.with_workers(*workers);
    }
    
    if let Some(interval) = matches.get_one::<u64>("update-interval") {
        config = config.with_update_interval(*interval);
    }

    println!("[@] Using {} worker threads", config.num_workers);

    // Start worker pool
    let mut pool = WorkerPool::new(config);
    pool.start()?;

    // Start input monitoring thread
    start_input_monitor();

    // Run the main loop
    let result = pool.run();

    // Shutdown
    if RUNNING.load(Ordering::Relaxed) {
        println!("[!] Shutting down...");
    }
    
    pool.shutdown()?;
    result
}

fn setup_signal_handler() {
    ctrlc::set_handler(move || {
        println!("\n[!] Received interrupt signal, shutting down...");
        RUNNING.store(false, Ordering::Relaxed);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
}

fn start_input_monitor() {
    thread::spawn(|| {
        if !atty::is(atty::Stream::Stdin) {
            println!("[!] Non-TTY environment detected. Keypress updates are disabled.\n");
            return;
        }

        println!("[i] Press Enter to see the current status:\n");
        
        let stdin = io::stdin();
        let mut input = String::new();
        
        while RUNNING.load(Ordering::Relaxed) {
            input.clear();
            if stdin.read_line(&mut input).is_ok() {
                let (generated, found) = onion_generator::get_stats();
                let now = chrono::Local::now();
                println!("[@] {}: Generated {} addresses, Found {} addresses", 
                         now.format("%H:%M:%S"), generated, found);
            }
        }
    });
}

fn run_single_threaded_with_input(prefixes: &[String]) -> Result<()> {
    // Start input monitoring for single-threaded mode
    start_input_monitor();
    
    // Start stats reporting thread
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));
            if !RUNNING.load(Ordering::Relaxed) {
                break;
            }
            
            let (generated, found) = onion_generator::get_stats();
            let now = chrono::Local::now();
            println!("[@] {}: Generated {} addresses, Found {} addresses", 
                     now.format("%H:%M:%S"), generated, found);
        }
    });

    run_single_threaded(prefixes)
}

// Add atty and ctrlc dependencies to Cargo.toml
