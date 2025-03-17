use std::{
    env, fs::{self, DirBuilder},
    io::{stdin, stdout, Write},
        path::{Path, PathBuf},
        process::Command,
    };

use crossterm::{terminal::{self, EnterAlternateScreen}, ExecutableCommand};
const BLUE: &str = "\x1b[34m";     // Blue for directories
const GREEN: &str = "\x1b[32m";    // Green for executable files
const YELLOW: &str = "\x1b[33m";   // Yellow for symlinks
const CYAN: &str = "\x1b[36m";     // Cyan for special files
const RED: &str = "\x1b[31m";      // Red for errors
const BOLD: &str = "\x1b[1m";      // Bold text
const RESET: &str = "\x1b[0m";     // Reset formatting


pub struct firefile;

impl firefile {

    pub fn find_files(&self) {
        let mut stdout = stdout();
        //terminal::enable_raw_mode().unwrap();
        stdout.execute(EnterAlternateScreen).unwrap();
        let stdin = stdin();

        let mut curr_directory = PathBuf::from(r"E:\");

        self.list_files(&curr_directory);

        loop {
            print!("\nfirefile 🚀 <usage: cd <dirname>, back, vc, exit, mkfl (make file) , mkdir (make new folder)> > ");
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let args: Vec<&str> = input.trim().split_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            match args[0] {
                "exit" | "e" => {
                    println!("Bye bye... 😔!");
                    break;
                }
                "back" | "b" => {
                    if let Some(parent) = curr_directory.parent() {
                        curr_directory = parent.to_path_buf();
                    } else {
                        println!("{}----------Already at the root directory.----------{}", RED,RESET);
                    }
                }
                "vc" => {
                    self.open_process("code", &curr_directory);
                }
                "cd" => {
                    if args.len() < 2 {
                        println!("Usage: cd <directory_name>");
                        continue;
                    }
                    let new_path = curr_directory.join(args[1..].join(" "));
                    if new_path.is_dir() {
                        curr_directory = new_path;
                    } else {
                        println!("Invalid directory: {}", args[1]);
                    }
                }
                "help" =>{
                    println!(" cd --> change directory \n vc --> open vs code with current directory");
                }
                "mkdir"=>{
                    self.make_directory(&curr_directory,args[1..].join(" ").as_str());
                }
                "mkfl"=>{
                    let file = curr_directory.join(format!("{}", args[1..].join(" ").as_str()));
                    match fs::File::create(&file) {
                        Ok(_) => println!("{}File created successfully!{}", BLUE, RESET),
                        Err(err) => eprintln!("{}Error creating file:{} {}", RED, err, RESET),
                    }
                }
                _ => println!("{}----------Unknown command:{}----------{}", RED,args[0],RESET),
            }

            self.list_files(&curr_directory);
        }


        stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    }








    fn list_files(&self, path: &Path) {

        // Try to read the directory
        match fs::read_dir(path) {
            Ok(entries) => {
                // Collect and sort entries
                let mut files = Vec::new();
                let mut dirs = Vec::new();

                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let path = entry.path();
                            let filename = entry.file_name().to_string_lossy().to_string();

                            // Get metadata or skip with error message if unavailable
                            let metadata = match entry.metadata() {
                                Ok(meta) => meta,
                                Err(e) => {
                                    eprintln!("{}Warning: Unable to read metadata for {}: {}{}",
                                              YELLOW, filename, e, RESET);
                                    continue;
                                }
                            };

                            // Sort into appropriate category and format with icon
                            if metadata.is_dir() {
                                dirs.push(format!("{}{}{} {}{}", BLUE, BOLD, "📂", filename, RESET));
                            } else if metadata.is_symlink() {
                                files.push(format!("{}{} {}{} -> {}", YELLOW, "🔗", filename, RESET,
                                                  path.read_link().unwrap_or_default().display()));
                            } else if metadata.is_file() {
                                // Check if file is executable (simplified - for Unix-like systems)
                                #[cfg(unix)]
                                let is_executable = {
                                    use std::os::unix::fs::PermissionsExt;
                                    metadata.permissions().mode() & 0o111 != 0
                                };

                                #[cfg(not(unix))]
                                let is_executable = false;

                                if is_executable {
                                    files.push(format!("{}{} {}{}", GREEN, "🚀", filename, RESET));
                                } else {
                                    files.push(format!("📄 {}", filename));
                                }
                            } else {
                                // Special files like devices, etc.
                                files.push(format!("{}{} {}{}", CYAN, "📎", filename, RESET));
                            }
                        }
                        Err(e) => {
                            eprintln!("{}Warning: Failed to read entry: {}{}", YELLOW, e, RESET);
                        }
                    }
                }

                // Sort alphabetically (case-insensitive)
                dirs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                files.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

                // Combine dirs first, then files
                let all_entries = [dirs, files].concat();
                let total = all_entries.len();

                if total == 0 {
                    println!("Directory is empty.");
                    return;
                }

                // Calculate optimal column width based on longest filename
                let max_width = all_entries.iter()
                    .map(|name| name.len())
                    .max()
                    .unwrap_or(40);
                let col_width = max_width + 2; // Add some padding

                // Calculate optimal number of columns based on terminal width
                let term_width = match term_size::dimensions() {
                    Some((w, _)) => w,
                    None => 80,
                };
                let cols = std::cmp::max(1, term_width / col_width);

                // Print header
                println!("{}{}Contents of: {}{}{}", BOLD, BLUE, path.display(), RESET, BOLD);
                println!("{} items ({})", total, path.display());
                println!();

                // Print files in columns
                for (i, name) in all_entries.iter().enumerate() {
                    print!("{:<width$}", name, width = col_width);
                    if (i + 1) % cols == 0 || i == total - 1 {
                        println!();
                    }
                }
            }
            Err(err) => {
                eprintln!("{}Error reading directory '{}': {}{}",
                         RED, path.display(), err, RESET);
            }
        }
    }






    fn open_process(&self, command: &str, path: &PathBuf) {
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", command, "."])
                .current_dir(path)
                .spawn()
        } else {
            Command::new(command).arg(".").current_dir(path).spawn()
        };

        if let Err(err) = result {
            eprintln!("Failed to open process: {}", err);
        }else{
            println!("🚀VSCode launched successfully!🚀 \nAdios Amigos!");
            std::process::exit(0);
        }
    }







    fn make_directory(&self,path: &PathBuf,folder_name: &str){
        let dir = DirBuilder::new();
        let path = path.join(folder_name);
        match dir.create(path) {
            Ok(_) => println!("{}Directory created successfully!{}", BLUE, RESET),
            Err(err) => eprintln!("{}Error creating directory:{} {}", RED, err, RESET),
        }

    }

}
