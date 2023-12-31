use person_struct_parser::person_module::*;

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn person_to_string() -> anyhow::Result<()> {
        let person = Person {
            name: String::from("Davyd"),
            age: 20,
            city: String::from("Kryvyj Rig"),
            zip: 77649,
            phone: String::from("+380487586554"),
        };
        assert_eq!(person.to_string(), "Davyd 20 Kryvyj Rig77649 +380487586554");
        Ok(())
    }

    #[test]
    fn parsing_normal() -> anyhow::Result<()> {
        let person = parse("Davyd 20 Lviv75869 +380457763154")?;
        assert_eq!(person.to_string(), "Davyd 20 Lviv75869 +380457763154");
        Ok(())
    }

    #[test]
    fn parsing_unnormal_age() -> anyhow::Result<()> {
        let person = parse("Davyd 2000 Lviv75869+380457763154")?;
        assert_eq!(person.to_string(), "Davyd 0 Lviv75869 +380457763154");
        Ok(())
    }

    #[test]
    fn parsing_normal0() -> anyhow::Result<()> {
        let person = parse("Davyd 20 Win+/sto++/n-S?,?(al*)*e*m75869 +380457763154")?;
        assert_eq!(
            person.to_string(),
            "Davyd 20 Winston-Salem75869 +380457763154"
        );
        Ok(())
    }

    #[test]
    fn parsing_another_normal1() -> anyhow::Result<()> {
        let person = parse("Davyd20Kryvyj Rig75869 +380457763154")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Kryvyj Rig75869 +380457763154");
        Ok(())
    }
    #[test]
    #[should_panic]
    fn parsing_another_normal2() {
        let person = parse("5Davyd20Lviv +380457763154");
        assert_eq!(person.unwrap().to_string(), "Davyd 20 Lviv +380457763154");
    }
}
