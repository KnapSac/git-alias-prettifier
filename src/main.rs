use ansi_term::Colour::*;
use git2::{Config, Error};
use std::cmp;

fn main() -> Result<(), Error> {
    let global_config = Config::find_global()?;
    let global_config = Config::open(global_config.as_path())?;

    let mut max_alias_len = 0;

    let config_entries = &global_config.entries(Some("alias.*"))?;
    for entry in config_entries {
        let entry = entry?;
        let alias = entry.name().unwrap()[6..].to_string();
        max_alias_len = cmp::max(max_alias_len, alias.len());
    }

    println!("{}", Red.paint("Aliases:"));

    let config_entries = &global_config.entries(Some("alias.*"))?;
    for entry in config_entries {
        let entry = entry?;
        let alias = entry.name().unwrap()[6..].to_string();
        let command = entry.value().unwrap().to_string();

        println!(
            "  {}{} = {}",
            Green.paint(alias.to_string()),
            " ".repeat(max_alias_len - alias.len()),
            Blue.paint(command.to_string())
        );
    }

    Ok(())
}
