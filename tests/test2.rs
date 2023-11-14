use person_struct_parser::person_module::*;

#[cfg(test)]
mod difficult_tests {
    use super::*;

    #[test]
    fn parsing_unnormal() -> anyhow::Result<()> {
        let person = parse("Dav-+yd-2/0Lv/iv75hgf86l+9")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Lviv75869");
        Ok(())
    }
    #[test]
    fn parsing_unnormal3() -> anyhow::Result<()> {
        let person = parse("Dav-+yd-2/0Lv/iv75hgf86l+984")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Lviv00000");
        Ok(())
    }
    #[test]
    fn parsing_unnormal4() -> anyhow::Result<()> {
        let person = parse("Dav-+yd-2/0-terN+OP/iL75hgf8l+")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Ternopil00758");
        Ok(())
    }

    #[test]
    fn parsing_unnormal2() -> anyhow::Result<()> {
        let person = parse("Dav-+yd-2/0Lv/iv*75hgf86l+9")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Lviv75869");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn parsing_incorrect() {
        let person = parse("20 Davyd Lviv");
        assert_eq!(person.unwrap().to_string(), "Davyd 20 Lviv");
    }
    #[test]
    #[should_panic]
    fn parsing_incorrect5() {
        let person = parse("20 Davyd L v iv");
        assert_eq!(person.unwrap().to_string(), "Davyd 20 Lviv");
    }
}
