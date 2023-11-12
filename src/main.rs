use person_struct_parser::person_module::*;
extern crate clap;
use clap::{App, Arg};
use std::fs;

///This is the main function that works as CLI
pub fn main() -> Result<(), std::io::Error> {
    let matches = App::new("My CLI Program")
        .version("1.0")
        .author("Your Name")
        .about("Опис вашої програми")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Вкажіть вхідний файл")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches
        .value_of("input")
        .unwrap_or("ExampleInputForCLI.txt");
    let file_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => return Err(err),
    };

    match parse(&file_content) {
        Ok(result) => println!("Успішно розпарсено: {}", result),
        Err(err) => eprintln!("Помилка під час парсингу: {:?}", err),
    }
    Ok(())
}
