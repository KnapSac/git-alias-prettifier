use ansi_term::Colour::*;
use std::{cmp, env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        invalid_gitconfig_file();
    }

    let filename = &args[1];

    if !filename.ends_with(".gitconfig") {
        invalid_gitconfig_file();
    }

    let contents = fs::read_to_string(filename).expect("Unable to read gitconfig");
    let aliases_start = contents.find("[alias]").unwrap_or(0);
    let aliases_end = contents[aliases_start + 1..]
        .find('[')
        .unwrap_or(contents.len());
    let aliases: &str = &contents[aliases_start + 8..aliases_start + aliases_end];

    let mut output_lines = Vec::new();
    let mut max_alias_len = 0;
    for line in aliases.lines() {
        if line.starts_with('#') {
            output_lines.push((line, ""));
        } else {
            let line_parts: Vec<&str> = line.splitn(2, '=').collect();
            let alias = line_parts[0].trim();
            let command = line_parts[1].trim();
            output_lines.push((alias, command));
            max_alias_len = cmp::max(max_alias_len, alias.len());
        }
    }

    for (alias, command) in output_lines.iter() {
        if command.is_empty() {
            println!("{}", Red.paint(alias.to_string()));
        } else {
            println!(
                "  {}{} = {}",
                Green.paint(alias.to_string()),
                " ".repeat(max_alias_len - alias.len()),
                Blue.paint(command.to_string())
            );
        }
    }
}

fn invalid_gitconfig_file() {
    eprintln!("Please supply the path to a .gitconfig file");
    process::exit(1);
}
