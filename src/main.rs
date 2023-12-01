use std::process::exit;
extern crate exitcode;
// use exitcode::ExitCode;
use anyhow::{Context, Result};

use env_logger::Builder;
use log::{
    // debug,
    error,
    info,
    // trace,
    LevelFilter,
};
// use colored::Colorize;

use clap::{command, crate_authors, crate_description, crate_name, crate_version, Parser};
use std::time::Instant;

// TODO: find a way to avoid crate name here
use aoc_2023::*;

// {{{ args

#[derive(Parser)]
#[command(name = crate_name!())]
#[command(author = crate_authors!(", "))]
#[command(version = crate_version!())]
#[command(about = crate_description!())]
pub struct Options {
    /// log level
    #[clap(short, long, value_enum, default_value = "info")]
    log_level: Level,

    /// day
    #[clap(short, long, default_value = "1")]
    day: Day,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Download(Download),
    Execute(Execute),
    Benchmark(Benchmark),
}

// Retrieve data file for selected day
#[derive(Parser, Debug)]
struct Download {
    /// session cookie
    #[clap(short, long, default_value = "unset")]
    session: String,
}

// Run the given part for selected day
#[derive(Parser, Debug)]
struct Execute {
    /// part
    #[clap(short, long, default_value = "1")]
    part: u8,
}

// Run all parts of all days until the selected one
#[derive(Parser, Debug)]
struct Benchmark {
    /// only current day or all until this one
    #[clap(short, long, default_value = "false")]
    current: bool,

    /// number of games to run for each (day, part)
    #[clap(short, long, default_value = "1")]
    number: u128,
}

// }}}
// {{{ log

#[derive(clap::ValueEnum, Debug, Clone)]
enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl From<Level> for LevelFilter {
    fn from(i: Level) -> Self {
        match i {
            Level::Error => LevelFilter::Error,
            Level::Warn => LevelFilter::Warn,
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
        }
    }
}

// }}}

fn main() -> Result<()> {
    let args = Options::parse();

    let mut builder = Builder::from_default_env();
    builder
        .filter(Some("curl"), LevelFilter::Error)
        .filter(None, LevelFilter::from(args.log_level))
        .init();

    let res = match args.command {
        Command::Download(o) => match get_data_server(args.day, &o.session) {
            Ok(..) => exitcode::OK,
            Err(e) => {
                error!("cannot retrieve data for day {}: {}", args.day, e);
                exitcode::IOERR
            }
        },
        Command::Execute(o) => {
            let res = solve(args.day, o.part, "").context("solving day")?;
            info!("day {} part {} solve: {}", args.day, o.part, res);
            exitcode::OK
        }
        Command::Benchmark(o) => {
            let start_day = if o.current { args.day } else { 1 };
            let mut acc = 0;
            for day in start_day..=args.day {
                for part in [1, 2] {
                    let start = Instant::now();
                    for _ in 0..o.number {
                        let _res = solve(day, part, "");
                    }
                    let elapsed = start.elapsed();
                    let elapsed = elapsed.as_micros() / o.number;
                    acc += elapsed;
                    info!("day {:2} part {} elapsed: {:10}µs", day, part, elapsed);
                }
            }
            info!("        total elapsed: {:10}µs", acc);
            exitcode::OK
        }
    };

    exit(res);
}
