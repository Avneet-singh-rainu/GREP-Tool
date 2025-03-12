use std::{env, fs, io::{stdin, stdout, Write}, path::Path};
use walkdir::WalkDir;

pub struct firefile {}

impl firefile {

    pub fn findfiles(&self) {

        let mut stdout = stdout();
        let stdin = stdin();
        let mut curr_directory = match env::current_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("{}", err);
                panic!("No directory found...");
            }
        };

        self.list_files(&curr_directory);

        loop {
            print!("\nfirefile 🚀 <usage: cd <dirname>, back, exit> > ");
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let args: Vec<&str> = input.trim().split_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            match args[0] {
                "exit" => {
                    println!("Bye bye... 😔!");
                    break;
                }
                "back" => {
                    if let Some(parent) = curr_directory.parent() {
                        curr_directory = parent.to_path_buf();
                    } else {
                        println!("Already at the root directory.");
                    }
                }
                "cd" => {
                    if args.len() < 2 {
                        println!("Usage: cd <directory_name>");
                        continue;
                    }
                    let new_path = curr_directory.join(args[1]);
                    if new_path.is_dir() {
                        curr_directory = new_path;
                    } else {
                        println!("Invalid directory: {}", args[1]);
                    }
                }  
                _ => println!("Unknown command: {}", args[0]),
            }
            self.list_files(&curr_directory);
        }
    }

    pub fn list_files(&self, path: &Path) {
        println!("\nContents of {:?}:", path);
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let metadata = entry.metadata().unwrap();
                        if metadata.is_dir() {
                            println!("📁 {}", entry.file_name().to_string_lossy());
                        } else {
                            println!("📄 {}", entry.file_name().to_string_lossy());
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error reading directory: {}", err),
        }
    }


}
