use person_struct_parser::person_module::*;
extern crate clap;
use anyhow::anyhow;
use clap::{App, Arg};
use std::fs;

///This is the main function that works as CLI
pub fn main() -> anyhow::Result<()> {
    let my_version = &*format!("Version: {}", env!("CARGO_PKG_VERSION"));
    let my_author = &*format!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    let matches = App::new("CLI Program")
        .version(my_version)
        .author(my_author)
        .about("This CLI is for parsing content of the file")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Enter your input file. If it is incorrect then the default file will be parsed")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches
        .value_of("input")
        .unwrap_or("ExampleInputForCLI.txt");
    let file_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => return Err(anyhow!(err)),
    };

    match parse(&file_content) {
        Ok(result) => {
            println!("CLI program");
            println!("{}", my_version);
            println!("{}", my_author);
            println!("This CLI is for parsing content of the file");
            println!("Parsed successfully");
            println!("Parsed: {}", result);
        }
        Err(err) => {
            eprintln!("Error during the parsing: {:?}", err);
            return Err(anyhow!(err));
        }
    }
    Ok(())
}
