// src/file_walk.rs
use std::path::{Path, PathBuf};
use directories::UserDirs;
use walkdir::WalkDir;

const MAX_DIRECTORY_WALK_DEPTH: usize = 3
;

/// Main entry point - discovers all valid user directories
/// Returns Vec of directories where files can be scattered
pub fn discover_user_directories() -> Vec<PathBuf> {
    
    let mut user_dirs: Vec<PathBuf> = Vec::new();

    let base_user_dirs: Vec<PathBuf> = get_base_user_dirs()
        .expect("Failed to collect base user directories");

    for base_dir in base_user_dirs {
        let mut walked_directories = walk_directory(&base_dir, MAX_DIRECTORY_WALK_DEPTH);
        user_dirs.append(&mut walked_directories);
    }

    return user_dirs;
}

/// Gets base user directories (Desktop, Documents, etc.)
/// Filters out None values (directories that don't exist)
fn get_base_user_dirs() -> Option<Vec<PathBuf>> {

    let user_dirs = UserDirs::new()?;
    let targets = vec![
        user_dirs.desktop_dir(),    // Desktop
        user_dirs.document_dir(),   // Documents  
        user_dirs.download_dir(),   // Downloads
        user_dirs.picture_dir(),    // Pictures
        user_dirs.video_dir(),      // Videos
        user_dirs.audio_dir(),      // Music
        ]
        .into_iter()
        .filter(|dir| dir.is_some())
        .map(|dir| dir.unwrap())
        .map(|dir| dir.to_path_buf())
        .collect();

    Some(targets)
}

/// Walks directory tree up to max_depth levels
/// Returns all discovered directories including base
fn walk_directory(base: &Path, max_depth: usize) -> Vec<PathBuf> {
    WalkDir::new(base)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect()
    
}