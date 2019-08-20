use regex::Regex;

pub struct DropQuery {
    table: String,
}

impl DropQuery {
    pub fn new(text: &str) -> Result<Box<DropQuery>, String> {
        let regex = Regex::new(
            r"
        (i?:drop[[:space:]]+table[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]*&",
        )
        .unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),
            Some(matched) => match matched.name("table") {
                Some(table) => Ok(Box::new(DropQuery {
                    table: String::from(table.as_str()),
                })),
                _ => Err(String::from("")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::DropQuery;
    #[test]
    fn missing_from_word_in_drop_statement() {
        assert!(DropQuery::new("drop 'a_table';").is_err());
    }
}
