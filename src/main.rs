use clap::Parser;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::io::{Write, Result};
use std::fs::{self, OpenOptions, File};
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

fn append_to_log(log_path: &Path, created: &Path) -> Result<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(log_path)?;

    writeln!(file, "{}", created.display())?;


    file.flush()?;

    Ok(())
}

/// Generates a random filename with .txt extension
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

/// Creates an empty file at the given path, then logs it
/// Returns Result - can fail if file creation or logging fails
fn create_file_at_path(file_path: &Path, log_path: &Path) -> Result<()> {
    // TODO:
    // 1. Create empty file at file_path using File::create()
    // 2. Use append_to_log() to log the file_path
    // 3. Return Ok(()) on success
    // Hint: Both operations return Result, use ? operator
    File::create(file_path)?;

    append_to_log(log_path, file_path)?;

    return Ok(());
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
        
        let rand_file_name: String = generate_random_filename();
        println!("Generated a random filename: {}", rand_file_name);

        let rand_file_path: PathBuf = PathBuf::from("C:/projects/file_scat").join(&rand_file_name);
        println!("Storing file at path: {}", rand_file_path.display());

        let log: PathBuf = log_path();
        create_file_at_path(&rand_file_path, &log)
            .expect("Failed to create a test file");

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