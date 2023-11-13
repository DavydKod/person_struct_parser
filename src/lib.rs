///Main module that contains all useful elements like Person struct, parse func etc
pub mod person_module {
    use thiserror::Error;

    ///My error type using thiserror to handle error situations
    #[derive(Error, Debug)]
    pub enum MyError {
        ///Global error of parsing
        #[error("An error occurred: {0}")]
        PSPError(String),
        ///Error for the situation when the field from input string is incorrect
        #[error("An error occurred incorrect field: {0}")]
        IncorrectField(String),
    }

    ///A struct to contain the information about a person such as name, age,city and zip
    pub struct Person {
        ///name of the person
        pub name: String,
        ///age of the person
        pub age: u32,
        ///city where the person lives
        pub city: String,
        ///zip
        pub zip: u32,
        ///if zip is Ukrainian
        pub zip_is_ua: bool,
    }
    impl std::fmt::Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut norm_zip: String = self.zip.to_string();
            if norm_zip.len() != 5 {
                while norm_zip.len() < 5 {
                    norm_zip = String::from("0").to_string() + &norm_zip.to_string();
                }
                if norm_zip.len() > 5 {
                    norm_zip = String::from("00000");
                }
            }
            write!(f, "{}-{}-{}{}", self.name, self.age, self.city, norm_zip)
        }
    }

    impl Person {
        ///A function to reduce Person object to normal view
        pub fn normalize(&mut self) -> &mut Self {
            if !self.name.is_empty() {
                let s = self.name.chars().next().unwrap().to_ascii_uppercase();
                self.name.make_ascii_lowercase();
                self.name.remove(0);
                self.name.insert(0, s);
            }
            if !self.city.is_empty() {
                let s = self.city.chars().next().unwrap().to_ascii_uppercase();
                self.city.make_ascii_lowercase();
                self.city.remove(0);
                self.city.insert(0, s);
            }
            if self.zip > 99999 {
                self.zip = 0;
            }
            //range of Ukrainian zip
            self.zip_is_ua = !(self.zip < 7000 || self.zip > 98999);
            self
        }
    }
    ///Main function to parse String object into the Person object(with normalization)
    pub fn parse(string: &str) -> Result<Person, MyError> {
        let mut tname = String::from("");
        let mut tage = String::from("");
        let mut tcity = String::from("");
        let mut tzip = String::from("");

        let mut has_name = false;
        let mut has_age = false;
        let mut has_city = false;
        let mut has_zip = false;

        for s in String::from(string).chars() {
            if !has_name {
                if s.is_ascii_alphabetic() {
                    has_name = true;
                }
            } else if has_name && !has_age {
                if s.is_ascii_digit() {
                    has_age = true;
                }
            } else if has_age && !has_city && s.is_ascii_alphabetic() {
                has_city = true;
            } else if has_city && !has_zip && s.is_ascii_digit() {
                has_zip = true;
            }
        }

        if !has_age {
            return Err(MyError::IncorrectField("age".to_string()));
        } else if !has_city {
            return Err(MyError::IncorrectField("city".to_string()));
        } else if !has_name {
            return Err(MyError::IncorrectField("name".to_string()));
        } else if !has_zip {
            return Err(MyError::IncorrectField("zip".to_string()));
        }

        let mut must_be_name = true;
        let mut must_be_age = false;
        let mut must_be_city = false;
        let mut must_be_zip = false;

        for s in String::from(string).chars() {
            if !s.is_ascii_digit() && !s.is_ascii_alphabetic() {
            } else if must_be_name {
                if s.is_ascii_alphabetic() {
                    tname.push(s);
                } else {
                    if s.is_ascii_digit() {
                        tage.push(s);
                    }
                    must_be_name = false;
                    must_be_age = true;
                }
            } else if must_be_age {
                if s.is_ascii_digit() {
                    tage.push(s);
                } else {
                    if s.is_ascii_alphabetic() {
                        tcity.push(s);
                    }
                    must_be_age = false;
                    must_be_city = true;
                }
            } else if must_be_city {
                if s.is_ascii_alphabetic() {
                    tcity.push(s);
                } else {
                    must_be_city = false;
                    must_be_zip = true;
                    if s.is_ascii_digit() {
                        tzip.push(s);
                    }
                }
            } else if must_be_zip && s.is_ascii_digit() {
                tzip.push(s);
            }
        }

        if tage.is_empty() {
            return Err(MyError::IncorrectField("age".to_string()));
        } else if tcity.is_empty() {
            return Err(MyError::IncorrectField("city".to_string()));
        } else if tname.is_empty() {
            return Err(MyError::IncorrectField("name".to_string()));
        } else if tzip.is_empty() {
            return Err(MyError::IncorrectField("zip".to_string()));
        }

        let unnorm_age = tage.parse::<u32>();
        let unnorm_zip = tzip.parse::<u32>();
        let norm_age: u32 = match unnorm_age {
            Ok(value) => value,
            Err(_) => {
                let err = MyError::PSPError("Invalid age parsing".to_string());
                return Err(err);
            }
        };

        let norm_zip: u32 = match unnorm_zip {
            Ok(value) => value,
            Err(_) => {
                let err = MyError::PSPError("Invalid age parsing".to_string());
                return Err(err);
            }
        };
        let mut person = Person {
            name: tname,
            age: norm_age,
            city: tcity,
            zip: norm_zip,
            zip_is_ua: true,
        };
        person.normalize();
        Ok(person)
    }
}
