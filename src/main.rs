use clap::Parser;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

mod activities;
mod config;
#[allow(dead_code)]
mod domain;
#[allow(dead_code)]
mod experimental;
mod generators;
mod types;
use config::SessionConfig;
use domain::EventEnvelope;
use types::{Complexity, DevelopmentType, JargonLevel, OutputFormat};

/// A CLI tool that generates impressive-looking terminal output when stakeholders walk by
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Type of development activity to simulate
    #[arg(short, long, value_enum, default_value_t = DevelopmentType::Backend)]
    dev_type: DevelopmentType,

    /// Level of technical jargon in output
    #[arg(short, long, value_enum, default_value_t = JargonLevel::Medium)]
    jargon: JargonLevel,

    /// How busy and complex the output should appear
    #[arg(short, long, value_enum, default_value_t = Complexity::Medium)]
    complexity: Complexity,

    /// Duration in seconds to run (0 = run until interrupted)
    #[arg(short = 'T', long, default_value_t = 0)]
    duration: u64,

    /// Show critical system alerts or issues
    #[arg(short, long, default_value_t = false)]
    alerts: bool,

    /// Simulate a specific project
    #[arg(short, long, default_value = "distributed-cluster")]
    project: String,

    /// Use less colorful output
    #[arg(long, default_value_t = false)]
    minimal: bool,

    /// Show team collaboration activity
    #[arg(short, long, default_value_t = false)]
    team: bool,

    /// Simulate a specific framework usage
    #[arg(short = 'F', long, default_value = "")]
    framework: String,

    /// Seed the scheduler and family selection for deterministic replay
    #[arg(long)]
    seed: Option<u64>,

    /// Output mode
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    output_format: OutputFormat,

    /// Disable ANSI decoration
    #[arg(long, default_value_t = false)]
    no_color: bool,

    /// Emit trace events for scheduling decisions
    #[arg(long, default_value_t = false)]
    trace: bool,

    /// Print enum values and generator families as JSON, then exit
    #[arg(long, default_value_t = false)]
    list_values: bool,
}

fn main() {
    let args = Args::parse();

    if args.list_values {
        println!(
            "{}",
            serde_json::to_string_pretty(&activities::list_values_json())
                .expect("list-values payload should serialize")
        );
        return;
    }

    let config = SessionConfig {
        dev_type: args.dev_type,
        jargon_level: args.jargon,
        complexity: args.complexity,
        alerts_enabled: args.alerts,
        project_name: args.project,
        minimal_output: args.minimal,
        team_activity: args.team,
        framework: args.framework,
        seed: args.seed,
        output_format: args.output_format,
        no_color: args.no_color || std::env::var_os("NO_COLOR").is_some(),
        trace_enabled: args.trace,
    };

    let running = Arc::new(AtomicBool::new(true));
    let interrupt_flag = running.clone();

    ctrlc::set_handler(move || {
        interrupt_flag.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut rng = match config.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => {
            let mut system = rand::rng();
            StdRng::from_rng(&mut system)
        }
    };

    let start_time = Instant::now();
    let target_duration = if args.duration > 0 {
        Some(Duration::from_secs(args.duration))
    } else {
        None
    };

    let mut sequence = 0_u64;

    if matches!(config.output_format, OutputFormat::Json) {
        sequence += 1;
        print_event(&config, &activities::boot_event(&config, sequence));
    } else {
        for line in activities::intro_lines(&config) {
            println!("{line}");
        }
    }

    while running.load(Ordering::SeqCst) {
        if let Some(duration) = target_duration {
            if start_time.elapsed() >= duration {
                break;
            }
        }

        let events = activities::emit_cycle(&config, &mut rng, &mut sequence);
        for event in events {
            print_event(&config, &event);
        }

        thread::sleep(Duration::from_millis(rng.random_range(120..280)));
    }

    sequence += 1;
    let reason = if running.load(Ordering::SeqCst) {
        "duration-elapsed"
    } else {
        "interrupted"
    };
    print_event(&config, &activities::termination_event(sequence, reason));
}

fn print_event(config: &SessionConfig, event: &EventEnvelope) {
    match config.output_format {
        OutputFormat::Text => activities::print_text_event(config, event),
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string(event).expect("event serialization should not fail")
        ),
    }
}
