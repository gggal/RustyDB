use parser::constants::DROP_QUERY_REGEX;

pub struct DropQuery {
    table: String,
}

impl DropQuery {
    pub fn new(text: &str) -> Result<Box<DropQuery>, &'static str> {
       
        match DROP_QUERY_REGEX.captures(&text) {
            None => Err("Given query doesn't conform to the supported syntax."),
            Some(matched) => match matched.name("table") {
                Some(table) => Ok(Box::new(DropQuery {
                    table: String::from(table.as_str()),
                })),
                _ => Err("Given query doesn't conform to the supported syntax."),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::DropQuery;
    #[test]
    fn missing_table_word_in_drop_statement() {
        assert!(DropQuery::new("drop a_table;").is_err());
    }

     #[test]
    fn correct_table_name_is_accuired() {
        let query = DropQuery::new("drop table a_table;").unwrap();
        assert_eq!(query.table, "a_table");
    }
}
