# person_struct_parser

## Parser for Rust source code

- GITHUB: https://github.com/DavydKod/person_struct_parser
- CRATES.IO: https://crates.io/crates/person_struct_parser

Person_struct_parser(PSP) is a parsing library for parsing a String into a person object.

- PSP has structure **Person**(`person_struct_parser::person_module::Person`) for containing the information about a person(name,age,city,zip) and zip_is_ua(true if zip is UA)

```rust
pub struct Person {
        pub name: String,
        pub age: u32,
        pub city: String,
        pub zip: u32,
        pub zip_is_ua: bool,
    }
```

- There is a function `person_struct_parser::person_module::normalize` implemented for **Person** to reduce object data to **normal form**:

```rust
pub fn normalize(&mut self) -> &mut Self
```

- Function `person_struct_parser::person_module::parse` is implemented for **Person**, it's main method for parsing **String** into the **Person** object with normalization:

```rust
pub fn parse(string: &str) -> anyhow::Result<Person>
```

- `std::fmt::Display` is implemented for **Person**

- **Grammar** for parsing:

```pest
low_alpha = {'a'..'z'}
high_alpha = {'A'..'Z'}
digit = {'0'..'9'}

name = {high_alpha ~ low_alpha+}
age = {digit{1,4}}
city = {high_alpha ~ ((low_alpha+) | (low_alpha+ ~ ('-' | ' ') ~ low_alpha+))}
zip = {digit{5}}

person = {name ~ ' ' ~ age ~ ' ' ~ city ~ zip}
```

## Example

- **Normalization**. To normalize person object. Next example will print `Roman-21-Paris54586`:

```rust
let mut person = Person{name:String::from("RoMAn"),age:21,city:String::from("PaRiS"),zip:54586};
println!("{}",person.normalize());
```

- **Parsing**. Next example will print `Roman-21-Paris54586` because of parsing and normalization after:

```rust
println!("{}",parse("-+Ro*Ma/N//2*+-1..PaR*I-s-54+gh5h-+h8ghj6").unwrap());
```

- **CLI**. You can execute `cargo run -- -i your_file_name.txt` in command prompt to parse the content of your file. If there is a problem it will parse the appropriate default file. Also there are more commands - try `cargo run -- --help` for more info.

## Custom Error

There is a custom error enum type using `thiserror` crate for parsing. For more look the documentation
