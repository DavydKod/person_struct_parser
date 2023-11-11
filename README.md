# person_struct_parser

# Parser for Rust source code

- GITHUB: https://github.com/DavydKod/person_struct_parser
- CRATES.IO: https://crates.io/crates/person_struct_parser

Person_struct_parser(PSP) is a parsing library for parsing a String into a person object.

- PSP has structure **Person**(`person_struct_parser::person_module::Person`) for containing the information about a person(name,age,city)

```rust
pub struct Person {
        pub name: String,
        pub age: u32,
        pub city: String,
    }
```

- Function `person_struct_parser::person_module::parse` is implemented for **Person**, it's main method for parsing **String** into the **Person** object:

```rust
pub fn parse(string: &str) -> anyhow::Result<Person>
```

- `std::fmt::Display` is implemented for **Person**
- There is function `person_struct_parser::person_module::normalize` implemented for **Person** to reduce object data to **normal form**:

```rust
pub fn normalize(&mut self) -> &mut Self
```

- The **grammar** for parsing is placed in `grammar.pest` file in `src` folder.
