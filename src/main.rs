mod friday;

use std::path::Path;

use chrono::{Datelike, NaiveDate, Weekday};
use clap::Parser;

use crate::friday::Friday;

#[derive(Debug, Clone, Copy)]
struct Repo;

// https://crates.io/crates/vfs
#[derive(Clone, Copy)]
struct Filesystem;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Cli {
    date: NaiveDate,
}

fn checkout_last_before_date(repo: Repo, date: NaiveDate) -> Filesystem {
    todo!()
}

fn pure_main(a: Filesystem, b: Filesystem) -> String {
    todo!()
}

fn main() {
    let Cli { date } = Cli::parse();
    let this_friday = Friday::try_from(date).unwrap();
    let last_friday = this_friday.previous_friday();
    let repo_path = &Path::from(std::env::var("NIXPKGS_CLONE_PATH"));
    let repo = Repo::new(repo_path).unwrap();

    let checkout_friday_last_week = checkout_last_before_date(repo, friday_last_week);
    let checkout_friday_this_week = checkout_last_before_date(repo, friday_this_week);
    let output = pure_main(checkout_friday_last_week, checkout_friday_this_week);
    println!("{output}");
}
