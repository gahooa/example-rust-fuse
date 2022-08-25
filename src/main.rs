mod cli;
mod filesystem;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    filesystem::AppCoveFS::launch(&args).unwrap()
}
