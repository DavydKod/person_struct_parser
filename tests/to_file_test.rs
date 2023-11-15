use person_struct_parser::person_module::*;

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn write_parsed_person_to_file() -> anyhow::Result<()> {
        write_to_file("Example.txt", "Mi*/-ke3+3+New* Y/o*rK*.45s6ssx-+75+");
        Ok(())
    }
}
