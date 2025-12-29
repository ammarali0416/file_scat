mod art;

use clap::Parser;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::io::{Write, Result};
use std::fs::{self, OpenOptions, File, read_to_string, remove_file};
use rand::Rng;
use art::ASCII_ART;


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
    // Flag to enable test mode
    #[arg(long, )]
    testing: bool,
}

/// Creates file at path and writes ASCII art content
fn create_file_with_content(file_path: &Path) -> Result<()> {
    let mut file: File = File::create(file_path)?;
    
    file.write_all(ASCII_ART.as_bytes())?;
    file.flush()?;
    Ok(())
}
fn append_to_log(log_path: &Path, file_paths: Vec<PathBuf>) -> Result<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(log_path)?;

    for path in file_paths.iter() {
        writeln!(file, "{}", path.display())?;
    }

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

fn log_path() -> PathBuf {
    let proj = ProjectDirs::from("com", "Ammar_Ali", "file_scat")
    .expect("Could not determine a per-user data directory");

    let dir = proj.data_local_dir();

    fs::create_dir_all(dir).expect("Could not create log directory)");

    let log_path = dir.join("created_files.log");
    
    return log_path;
}

/// Reads log file, attempts to delete each file listed
/// Handles missing files gracefully
fn cleanup_files(log_path: &Path) -> Result<()> {
    
    let log_lines = read_to_string(log_path)?;

    println!("Reading log file at: {}", log_path.display());

    let file_paths = log_lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string());

    for file in file_paths {
        let file_path: PathBuf = PathBuf::from(file);
        if let Err(e) = remove_file(&file_path) {
            eprintln!("Could no delete file: {}\n {}", file_path.display(), e)
        }
    }

    fs::write(log_path, "")?;

    Ok(())
}

fn main() {
    let args = Args::parse();
    
    if args.testing {
        println!("Testing mode: Args: {:?}", args);

        let log: PathBuf = log_path();
        
        let mut file_paths_array: Vec<PathBuf> = Vec::with_capacity(args.count);

        for _i in 1..args.count {

            let rand_file_name: String = generate_random_filename();
            //println!("Generated a random filename: {}", rand_file_name);
    
            let rand_file_path: PathBuf = PathBuf::from("C:/projects/file_scat").join(&rand_file_name);
            //println!("Storing file at path: {}", rand_file_path.display());
            
            create_file_with_content(&rand_file_path)
                .expect("Failed to create a random file");
        
            file_paths_array.push(rand_file_path);
            
        }

        println!("{} files created.", file_paths_array.len());

        append_to_log(&log, file_paths_array).expect("Failed to log files");

        return;
    }
    
    let log = log_path();
    
    if args.cleanup {
        println!("Mode: cleanup");
        println!("Log file: {}", log.display());

        cleanup_files(&log).expect("Cleanup failed");
        println!("Cleanup complete");        
    } else {
        todo!();
    }
}