use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Stdout, Write},
};
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct MyGrep;

impl MyGrep {
    pub fn do_color(&self, line: &str, regex: &Regex, stdout: &mut StandardStream) {
        let mut last_end = 0;

        for mat in regex.find_iter(line) {
            let (start, end) = (mat.start(), mat.end());
            write!(stdout, "{}", &line[last_end..start]).unwrap();
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
            write!(stdout, "{}", &line[start..end]).unwrap();
            stdout.reset().unwrap();

            last_end = end;
        }

        // Print remaining part of the line
        writeln!(stdout, "{}", &line[last_end..]).unwrap();
    }

    pub fn process(&self, regex: Regex, path: &String, stdout: &mut StandardStream) {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Error opening file: {}", path);
                return;
            }
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if regex.is_match(&line) {
                        self.do_color(&line, &regex, stdout);
                    } else {
                        writeln!(stdout, "{}", line).unwrap();
                    }
                }
                Err(_) => {
                    eprintln!("Error reading line from file.");
                }
            }
        }
    }

    pub fn run(&self) {
        println!("my grep is running...");
        let stdin = stdin();
        let mut stdout = stdout();
        let mut input = String::new();

        loop {
            print!("CMD> ");
            stdout.flush().unwrap();
            input.clear();

            if stdin.read_line(&mut input).is_err() {
                eprintln!("Error reading input.");
                continue;
            }

            let args: Vec<&str> = input.trim().split_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            if args[0] == "exit" {
                println!("bye bye...");
                break;
            }

            if args.len() < 2 {
                eprintln!("Insufficient arguments provided. Usage: <pattern> <file>");
                continue;
            }

            let pattern = args[0];

            let path = args[1..].join(" ");
            let path:String = path.trim_matches('"').to_string();

            let regex = match Regex::new(pattern) {
                Ok(re) => re,
                Err(e) => {
                    eprintln!("Error in regex pattern: {}", e);
                    continue;
                }
            };

            //  standardStream for colored output
            let mut color_stdout = StandardStream::stdout(ColorChoice::Always);
            self.process(regex, &path, &mut color_stdout);
        }
    }
}
