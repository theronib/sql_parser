use anyhow::Result;
use pest::Parser;
use my_sql_parser::*;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("--help") => {
            print_help();
            return Ok(());
        }
        Some("--credits") => {
            print_credits();
            return Ok(());
        }
        Some(file_path) => {
            let content = fs::read_to_string(file_path)?;
            parse_file(&content);
        }
    }
    Ok(())
}

fn parse_file(content: &str) {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parsed = Grammar::parse(Rule::query, line)
            .or_else(|_| Grammar::parse(Rule::insert, line))
            .or_else(|_| Grammar::parse(Rule::condition, line))
            .or_else(|_| Grammar::parse(Rule::multiple_conditions, line))
            .or_else(|_| Grammar::parse(Rule::order_by, line));

        match parsed {
            Ok(pairs) => println!("Parsed: {:?}", pairs),
            Err(_) => eprintln!("Error: cannot parse line: {}", line),
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("  sql_parser <FILE>     Parse the given SQL file");
    println!("  sql_parser --help     Show this help message");
    println!("  sql_parser --credits  Show author information");
}

fn print_credits() {
    println!("SQL Parser");
    println!("Author: Yaroslav Baranivskyy");
}
