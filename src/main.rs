use clap::Parser;
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fs::OpenOptions;
use rand::Rng;



#[derive(Parser, Debug)]
#[command(name = "file_scat")]
#[command(about = "Scatters dummy files and logs their paths; can cleanup via --cleanup")]
struct Args {
    /// Delete generated files listed in the log
    #[arg(long)]
    cleanup: bool,
    // Number of files to generate
    #[arg(long, default_value = "10")]
    count: u32,
    // Flag to enable test mode
    #[arg(long, )]
    testing: bool,
}

fn append_to_log(log_path: &Path, created: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(log_path)?;

    writeln!(file, "{}", created.display())?;


    file.flush()?;

    Ok(())
}

/// Generates a random filename with .txt extension
/// Format: random_{8_chars}.txt
fn generate_random_filename() -> String {

    let length: usize = 32;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    let chars: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return format!("{}.txt", chars);
}


fn log_path() -> PathBuf {
    let proj = ProjectDirs::from("com", "Ammar_Ali", "file_scat")
    .expect("Could not determine a per-user data directory");

    let dir = proj.data_local_dir();

    fs::create_dir_all(dir).expect("Could not create log directory)");

    let log_path = dir.join("created_files.log");
    
    return log_path;
}

fn main() {
    let args = Args::parse();
    
    if args.testing {
        println!("Testing mode: Args: {:?}", args);
        println!("Generated a random filename: {}", generate_random_filename());
        return;
    }
    
    let log = log_path();
    
    if args.cleanup {
        println!("Mode: cleanup");
        println!("Log file: {}", log.display());
    } else {
        println!("Mode: generate");
        println!("Count: {}", args.count);
        println!("Testing: {}", args.testing);
        println!("Log file: {}", log.display());
    
        let fake = std::path::PathBuf::from("some_fake_file.txt");
        append_to_log(&log, &fake).expect("Failed to append to log");
        println!("Appended to log: {}", log.display());
    }
}