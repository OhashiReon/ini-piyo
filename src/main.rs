use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process;

use colored::Colorize;

mod diff;
mod parser;

#[derive(Debug, Clone, Copy)]
enum Action {
    Check,
    Merge,
}

struct Config {
    base_path: String,
    target_path: String,
    action: Action,
}

impl Config {
    fn from_args(mut args: env::Args) -> Result<Self, String> {
        args.next();
        let mut base_path = None;
        let mut target_path = None;
        let mut action = Action::Check;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--merge" => action = Action::Merge,
                "--check" => action = Action::Check,
                _ if base_path.is_none() => base_path = Some(arg),
                _ if target_path.is_none() => target_path = Some(arg),
                unexpected => return Err(format!("Unexpected argument: {}", unexpected)),
            }
        }
        Ok(Config {
            base_path: base_path.ok_or("Missing base_file argument")?,
            target_path: target_path.ok_or("Missing target_file argument")?,
            action,
        })
    }
}

fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!("Usage: tool <base_file> <target_file> [--check|--merge]");
        process::exit(1);
    });

    let base_text = fs::read_to_string(&config.base_path).unwrap_or_else(|e| {
        eprintln!("Failed to read base file '{}': {}", config.base_path, e);
        process::exit(1);
    });

    let target_text = fs::read_to_string(&config.target_path).unwrap_or_else(|_| {
        println!("Target file '{}' not found. Proceeding with empty content.", config.target_path);
        String::new()
    });

    let diff_results = diff::diff(&base_text, &target_text);

    match config.action {
        Action::Check => run_check(&diff_results),
        Action::Merge => run_merge(&config.target_path, &diff_results),
    }
}

fn run_check(results: &[diff::DiffResult]) {
    let mut missing_count = 0;
    let mut empty_count = 0;

    for result in results {
        match &result.target {
            Some(target) => {
                if let (
                    parser::IniLine::KeyValue { value: base_val, .. },
                    parser::IniLine::KeyValue { value: target_val, .. }
                ) = (&result.base, target) {
                    if target_val.trim().is_empty() && !base_val.trim().is_empty() {
                        println!("{}  | {}", "*".yellow(), target.get_raw());
                        empty_count += 1;
                    } else {
                        println!("   | {}", target.get_raw());
                    }
                } else {
                    println!("   | {}", target.get_raw());
                }
            }
            None => {
                let line_str = match &result.base {
                    parser::IniLine::KeyValue { key, separator, .. } => {
                        format!("{}{}", key, separator.as_str())
                    }
                    _ => result.base.get_raw().to_string(),
                };
                println!("{}  | {}", "+".green(), line_str);
                missing_count += 1;
            }
        }
    }

    println!("\nCheck Complete.");
    if missing_count == 0 && empty_count == 0 {
        println!("{}", "No issues found.".green());
    } else{
        
        println!(" - Missing keys : {}", missing_count.to_string().green());
        println!(" - Empty vals   : {}", empty_count.to_string().yellow());
    
    }
}

fn run_merge(path: &str, results: &[diff::DiffResult]) {
    let file = File::create(path).unwrap_or_else(|e| {
        eprintln!("Failed to open '{}' for writing: {}", path, e);
        process::exit(1);
    });
    let mut writer = BufWriter::new(file);

    let mut added_count = 0;
    let mut empty_count = 0;

    for result in results {
        match &result.target {
            Some(target) => {
                if let (
                    parser::IniLine::KeyValue { value: base_val, .. },
                    parser::IniLine::KeyValue { value: target_val, .. }
                ) = (&result.base, target) {
                    if target_val.trim().is_empty() && !base_val.trim().is_empty() {
                        println!("{}  | {}", "*".yellow(), target.get_raw());
                        writeln!(writer, "{}", target.get_raw()).unwrap();
                        empty_count += 1;
                    } else {
                        println!("   | {}", target.get_raw());
                        writeln!(writer, "{}", target.get_raw()).unwrap();
                    }
                } else {
                    println!("   | {}", target.get_raw());
                    writeln!(writer, "{}", target.get_raw()).unwrap();
                }
            }
            None => {
                let line_str = match &result.base {
                    parser::IniLine::KeyValue { key, separator, .. } => {
                        format!("{}{}", key, separator.as_str())
                    }
                    _ => result.base.get_raw().to_string(),
                };
                println!("{}  | {}", "+".green(), line_str);
                writeln!(writer, "{}", line_str).unwrap();
                added_count += 1;
            }
        }
    }

    println!("\nMerge Complete.");
    if added_count == 0 && empty_count == 0 {
        println!("{}", "No issues found.".green());
    } else{
        
        println!(" - Added keys : {}", added_count.to_string().green());
        println!(" - Empty vals   : {}", empty_count.to_string().yellow());
    
    }
}