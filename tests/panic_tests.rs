use person_struct_parser::person_module::*;

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unequal() {
        let _person: Person = parse("DavydLviv").unwrap();
    }
    #[test]
    #[should_panic]
    fn incorrect() {
        let _person: Person = parse("25").unwrap();
    }
}
