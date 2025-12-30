mod file_walk;
mod file_helper;
mod art;

use clap::Parser;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::fs;

use file_helper::FileScatterer;
use file_walk::discover_user_directories;

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


fn log_path() -> PathBuf {
    let proj = ProjectDirs::from("com", "Ammar_Ali", "file_scat")
    .expect("Could not determine a per-user data directory");

    let dir: &Path = proj.data_local_dir();

    fs::create_dir_all(dir).expect("Could not create log directory)");

    let log_path = dir.join("created_files.log");
    
    return log_path;
}

fn main() {
    let args = Args::parse();
    let log: PathBuf = log_path();

    let mut scatterer = FileScatterer::new(log);

    if args.cleanup {
        scatterer.cleanup().expect("Cleanup failed");
    } else {
        let dirs = discover_user_directories();
        scatterer.scatter_files(args.count, &dirs).expect("Scatter failed");
        scatterer.save_log().expect("Log failed");
    }
}