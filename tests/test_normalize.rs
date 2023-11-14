use person_struct_parser::person_module::*;

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn test_normalize() -> anyhow::Result<()> {
        let mut p: Person = Person {
            name: String::from("aNakIN"),
            age: 23,
            city: String::from("cORuSAnT"),
            zip: 51415,
        };
        assert_eq!(p.normalize().to_string(), "Anakin 23 Corusant51415");

        Ok(())
    }
}
