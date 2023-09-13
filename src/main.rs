use chrono::DateTime;
use clap::Parser;
use colored::Colorize;
use duration_string::DurationString;
use std::io::BufRead;

/// Highlight lines of program output based on the latency between them
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    /// Do not echo lines of input as they arrive; show highlighted output after the command finishes
    #[arg(short, long)]
    no_echo: bool,

    /// Don't highlight lines with latency below this threshold
    #[arg(short, long, default_value = "1ms")]
    min_latency: DurationString,

    /// Show the latency between lines in the given unit. Valid units are ns, us, ms, s, m, h, d
    #[arg(short, long, default_value = "ms")]
    latency_unit: String,
}

#[derive(Debug)]
struct AnnotatedLine {
    line: String,
    timestamp: DateTime<chrono::Utc>,
}

fn main() {
    let cli = Cli::parse();
    let min_latency = chrono::Duration::from_std(cli.min_latency.into()).unwrap();

    let start_time = chrono::Utc::now();
    let lines: Vec<AnnotatedLine> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .inspect(|line| {
            if !cli.no_echo {
                println!("{}", line);
            }
        })
        .map(|line| AnnotatedLine {
            line,
            timestamp: chrono::Utc::now(),
        })
        .collect();

    if lines.is_empty() {
        return;
    }

    let mut max_inter_line_latency = chrono::Duration::milliseconds(1);
    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let since_last_line = line.timestamp - lines[index - 1].timestamp;
        if since_last_line > max_inter_line_latency {
            max_inter_line_latency = since_last_line;
        }
    }

    let latency_display_width: usize = if max_inter_line_latency > chrono::Duration::zero() {
        let d: usize = duration_as_unit(max_inter_line_latency, &cli.latency_unit)
            .ilog10()
            .try_into()
            .unwrap();
        d + 1
    } else {
        1
    };

    for (index, line) in lines.iter().enumerate() {
        let since_last_line = if index == 0 {
            lines.first().unwrap().timestamp - start_time
        } else {
            line.timestamp - lines[index - 1].timestamp
        };
        let red_intensity = if since_last_line > min_latency {
            let num = since_last_line.num_nanoseconds().unwrap() as f64;
            let denom = max_inter_line_latency.num_nanoseconds().unwrap() as f64;
            (255.0 * num / denom) as u8
        } else {
            0
        };
        let debug = if cli.debug {
            let max_lat_indicator = if since_last_line == max_inter_line_latency {
                "MAX".to_string()
            } else {
                "".to_string()
            };
            format!(
                "{} | R{} {} | ",
                max_lat_indicator, red_intensity, line.timestamp
            )
        } else {
            "".to_string()
        };
        println!(
            "{} {:>lat_width$}{} {} {}",
            "  ".on_truecolor(red_intensity, 0, 0),
            duration_as_unit(since_last_line, &cli.latency_unit),
            cli.latency_unit,
            debug,
            line.line,
            lat_width = latency_display_width
        );
    }
}

fn duration_as_unit(duration: chrono::Duration, unit: &str) -> u64 {
    match unit {
        "ns" => duration.num_nanoseconds().unwrap() as u64,
        "us" => duration.num_microseconds().unwrap() as u64,
        "ms" => duration.num_milliseconds() as u64,
        "s" => duration.num_seconds() as u64,
        "m" => duration.num_minutes() as u64,
        "h" => duration.num_hours() as u64,
        "d" => duration.num_days() as u64,
        _ => panic!("Unknown unit {}", unit),
    }
}
