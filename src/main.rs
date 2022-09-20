#![allow(unused)]

mod git_pull;
mod git_status;

use clap::Parser;
use git2::ConfigLevel::System;
use git2::{Error, Repository, StatusOptions};
use log;
use std::collections::BTreeMap;
use tracing_subscriber::fmt::{self, time::LocalTime};
use walkdir::WalkDir;

fn main() -> Result<(), Error> {
    let time_format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
            .expect("format string should be valid!");
    let timer = LocalTime::new(time_format);
    tracing_subscriber::fmt()
        // Configure formatting settings.
        .with_target(false)
        .with_timer(timer)
        .with_level(true)
        .with_line_number(true)
        .with_ansi(true)
        // Set the subscriber as the default.
        .init();
    let args = Args::parse();
    let path_buf = args.path;
    log::info!("path is {:?}", path_buf);

    let repository = match Repository::open(&path_buf) {
        Ok(repo) => repo,
        Err(e) => {
            log::error!("failed to open : {}", e);
            std::process::exit(exitcode::USAGE);
        }
    };

    git_pull::pull(&None, &None, &repository);

    let state = repository.state();
    log::info!("当前仓库状态 {:?}", state);

    let mut opts = StatusOptions::new();

    opts.include_ignored(true);
    let statuses = repository.statuses(Some(&mut opts))?;

    git_status::print_long(&statuses);

    WalkDir::new(path_buf)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| e.file_name().to_str().unwrap_or("").ends_with(".yml"))
        .for_each(|e| {
            log::info!("{}", e.path().display());
        });

    Ok(())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // /// Name of the person to greet
    // #[clap(short, long, value_parser)]
    // name: String,
    //
    // /// Number of times to greet
    // #[clap(short, long, value_parser, default_value_t = 1)]
    // count: u8,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
