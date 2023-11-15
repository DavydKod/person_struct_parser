use person_struct_parser::person_module::*;

#[cfg(test)]
mod difficult_tests {
    use super::*;

    #[test]
    fn parsing_unnormal() -> anyhow::Result<()> {
        let person = parse("Dav-yd-2/0Lv/iv75hgf86l9+3804fw5wef7wef-763-+1+54")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Lviv75869 +380457763154");
        Ok(())
    }
    #[test]
    fn parsing_unnormal3() -> anyhow::Result<()> {
        let person = parse("Dav-yd-2/0Lv/iv75hgf86l984 +38s04ssdd5f77kgufhj63154+++++++")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Lviv00000 +380457763154");
        Ok(())
    }
    #[test]
    fn parsing_unnormal4() -> anyhow::Result<()> {
        let person = parse("Dav-yd-2/0-terNOP/iL75hgf8l++3804y5 7yfy76ss3-+154")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Ternopil00758 +380457763154");
        Ok(())
    }

    #[test]
    fn parsing_unnormal2() -> anyhow::Result<()> {
        let person = parse("Dav-yd-2/0Lv/iv*75hgf86l9 +38045++++7763154")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd 20 Lviv75869 +380457763154");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn parsing_incorrect() {
        let person = parse("20 Davyd Lviv +380457763154");
        assert_eq!(person.unwrap().to_string(), "Davyd 20 Lviv +380457763154");
    }
    #[test]
    #[should_panic]
    fn parsing_incorrect5() {
        let person = parse("20 Davyd L v iv +3804 5 77 6 31 54");
        assert_eq!(person.unwrap().to_string(), "Davyd 20 Lviv +380457763154");
    }
}
