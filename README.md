# File Scatter Prank

## What does this do?
Upon opening the executable, the user goes through a simple command-line quiz with seemingly harmless questions that ultimately reveals the message "their name blew up!"

While this happens, a **detached background process** dumps 5000 files randomly across their main user directories. Each file contains ASCII art troll face. The background process continues even after the user exits the quiz.

## Why did I make this?
To learn Rust through a practical project, and to prank friends in the process.

## How does it work?

### Architecture
The program is built with a modular Rust architecture:

**Modules:**
- `main.rs` - Entry point and flow orchestration
- `prank.rs` - Interactive CLI questionnaire using `inquire` crate
- `file_helper.rs` - File operations encapsulated in `FileScatterer` struct
- `file_walk.rs` - Recursive directory discovery up to depth 3
- `setup.rs` - Log file initialization
- `art.rs` - ASCII art content
- `constants.rs` - Configuration values

**Process Flow:**
1. User runs executable (no arguments by double clicking the file)
2. Main process spawns detached child process with `--background-scatter` flag
3. Child process silently:
   - Discovers user directories (Desktop, Documents, Downloads, Pictures, Videos, Music)
   - Walks subdirectories up to 3 levels deep
   - Creates 5000 files with random 32-character alphanumeric names
   - Writes ASCII art to each file
   - Logs all paths to `AppData\Local\Ammar_Ali\file_scat\data\created_files.log`
4. Main process presents quiz:
   - "What's your name?" (text input)
   - "What's the color of the sky?" (select: Blew/Blue)
   - "What is the direction?" (select: Up/up)
5. Displays result: `{name} {color} {direction}! Get rekt nerd!`
6. User presses Enter, main process exits
7. Background process continues until all files are created

## How can I use this?
You can download the compiled binary from the [release page](https://github.com/ammarali0416/File-Scatter-Prank/releases) or clone the repo and compile it locally.


### Building from source
```bash
# Clone repository
cd file_scat

# Build release binary
cargo build --release

# Binary located at: target/release/file_scat.exe
```

### Running the prank
Simply execute the binary:
```bash
./target/release/file_scat.exe
```

Or double-click the executable on Windows.

### Cleanup
Delete all scattered files:
```bash
./file_scat.exe --cleanup
```

This reads the log file and removes all created files, then clears the log.

### Testing UI without scattering files
```bash
./file_scat.exe --test-ui
```

Hidden flag for development/testing.

### Configuration
Edit `src/constants.rs`:
- `PRANK_FILE_COUNT` - Number of files to create (default: 5000)
- `MAX_DIRECTORY_WALK_DEPTH` - Directory traversal depth (default: 3)

## Development process

This project was built using **pedagogical pair programming with Claude (Sonnet 4.5)** in learning mode.

**Approach:**
- Iterative development (1-3 files modified at a time)
- Skeleton code + hints provided, implementation done independently
- Errors encountered, debugged, and explained conceptually
- Rust ownership, borrowing, modules, traits taught through context
- Design decisions discussed before implementation

**Concepts learned:**
- Ownership vs borrowing (`&self`, `&mut self`, `self`)
- Module system and visibility (`mod`, `pub`, `crate::`)
- Error propagation (`?` operator vs `.expect()`)
- Iterator patterns (`.filter_map()`, `.collect()`)
- Struct methods vs associated functions
- Process spawning and detachment
- Type coercion (`PathBuf` → `&Path`)
- Shadowing vs mutation

**Tools used:**
- `clap` - CLI argument parsing
- `inquire` - Interactive prompts
- `rand` - Random generation
- `walkdir` - Directory traversal
- `directories` - Cross-platform user directories

Built as a learning project to learn Rust with hands on the keyboard, not straight copy-paste.

## License
See [License](https://github.com/ammarali0416/File-Scatter-Prank/blob/master/LICENSE). Use responsibly and at your own risk.