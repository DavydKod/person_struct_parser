# person_struct_parser

# Parser for Rust source code

GITHUB:[https://github.com/DavydKod/person_struct_parser]
CRATES.IO:[https://crates.io/crates/person_struct_parser]

Person_struct_parser(PSP) is a parsing library for parsing a String into a person object.

- PSP has structure **Person**[`person_struct_parser::person_module::Person`] for containing the information about a person(name,age,city)

```rust
pub struct Person {
        pub name: String,
        pub age: u32,
        pub city: String,
    }
```

- Person structure contains the information about name(String), age(u32) and city(String).

```rust

```

This method is implemented for Person to transform this into the String.

```rust
fn parse(string: &str) -> anyhow::Result<Person>
```

This method parse any &str into Person(under anyhow::Result<Person>). The grammar for this is written in ./grammar.pest file. Also there can be a lot of mistakes in the input &str and this parser will try to fix this and to create Person object.
