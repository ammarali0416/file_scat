use std::path::{Path, PathBuf};
use std::io::{Write, Result};
use std::fs::{self, OpenOptions, File, read_to_string, remove_file};
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::art::ASCII_ART;

/// Handles file scattering operations
pub struct FileScatterer {
    log_path: PathBuf,
    created_files: Vec<PathBuf>,
    rng: ThreadRng
}

impl FileScatterer {
    /// Constructor - creates new FileScatterer
    pub fn new(log_path: PathBuf) -> Self {
        Self 
        {   log_path: log_path,
            created_files: Vec::new(),
            rng: rand::thread_rng()
        }
    }
    
    /// Scatters count files across directories
    pub fn scatter_files(&mut self, count: usize, directories: &[PathBuf]) -> Result<()> {
        
        for _i in 0..count {
            let idx: usize = self.rng.gen_range(0..(directories.len()));
            let random_directory: &PathBuf = &directories[idx];

            let rand_file_name: String = self.generate_filename();    
            let rand_file_path: PathBuf = random_directory.join(&rand_file_name);
            
            self.create_file_with_content(&rand_file_path)?;
        
            self.created_files.push(rand_file_path);
            
        }

        Ok(())
    }
    
    /// Saves created files to log
    pub fn save_log(&self) -> Result<()> {
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&self.log_path)?;
    
        for path in self.created_files.iter() {
            writeln!(file, "{}", path.display())?;
        }
    
        file.flush()?;
    
        Ok(())    
    }
    
    /// Cleans up files listed in log
    pub fn cleanup(&self) -> Result<()> {
        let log_lines = read_to_string(&self.log_path)?;
    
        let file_paths = log_lines
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string());
    
        for file in file_paths {
            let file_path: PathBuf = PathBuf::from(file);
            if let Err(e) = remove_file(&file_path) {
                eprintln!("Could not delete file: {}\n {}", file_path.display(), e)
            }
        }
    
        fs::write(&self.log_path, "")?;
    
        Ok(()) 
    }
    
    /// Generates random filename
    fn generate_filename(&mut self) -> String {
        let length: usize = 32;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
        let chars: String = (0..length)
            .map(|_| {
                let idx = self.rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
    
        return format!("{}.txt", chars);    
    
    }

    /// Creates file at path and writes ASCII art content
    fn create_file_with_content(&self, file_path: &Path) -> Result<()> {
        let mut file: File = File::create(file_path)?;
        
        file.write_all(ASCII_ART.as_bytes())?;
        file.flush()?;
        Ok(())
    }
}