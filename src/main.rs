mod file_walk;
mod file_helper;
mod art;
mod setup;

use clap::Parser;
use std::path::PathBuf;
use file_helper::FileScatterer;

#[derive(Parser, Debug)]
#[command(name = "file_scat")]
#[command(about = "Scatters dummy files and logs their paths; can cleanup via --cleanup")]
struct Args {
    /// Delete generated files listed in the log
    #[arg(long)]
    cleanup: bool,
    // Number of files to generate
    #[arg(long, default_value = "10")]
    count: usize,
}

fn main() {
    let args = Args::parse();
    let log: PathBuf = setup::init_log_file();

    let mut scatterer = FileScatterer::new(log);

    if args.cleanup {
        scatterer.cleanup().expect("Cleanup failed");
    } else {
        let dirs = file_walk::discover_user_directories();
        scatterer.scatter_files(args.count, &dirs).expect("Scatter failed");
        scatterer.save_log().expect("Log failed");
    }
}