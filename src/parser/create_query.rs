use filter::Type;
use parser::constants::CREATE_QUERY_REGEX;

/// Represents a CREATE statement.
/// Contains a table name and a set of pairs for every column name and its corresponding type.
#[derive(Debug)]
pub struct CreateQuery {
    table: String,
    columns: Vec<(String, Type)>,
}

impl CreateQuery {
    /// Returns a create query object or an error message
    /// # Arguments
    ///
    /// * `text` - the query string that needs to be parsed
    ///
    /// # Example
    ///
    /// ```
    /// let create_query = CreateQuery::new("create table my_table");
    /// ```
    pub fn new(text: &str) -> Result<Box<CreateQuery>, &'static str> {
        match CREATE_QUERY_REGEX.captures(&text) {
            None => Err("Given query doesn't conform to the supported syntax."),
            Some(matched) => match (matched.name("table"), matched.name("cols")) {
                (Some(table), Some(cols)) => Ok(Box::new(CreateQuery {
                    table: String::from(table.as_str()),
                    columns: cols
                        .as_str()
                        .split(",")
                        .map(CreateQuery::str_to_col_tuple)
                        .collect(),
                })),
                _ => Err("Given query doesn't conform to the supported syntax."),
            },
        }
    }

    fn str_to_col_tuple(text: &str) -> (String, Type) {
        (
            String::from(text.split_whitespace().next().unwrap()),
            Type::parse(text.split_whitespace().last().unwrap()).expect("Unsupported type"),
        )
    }
}

#[cfg(test)]
mod tests {
    use parser::CreateQuery;

    #[test]
    fn missing_table_clause() {
        assert!(CreateQuery::new("create table_name;").is_err())
    }

    #[test]
    fn missing_columns_in_create() {
        assert!(CreateQuery::new("create table table_name;").is_err())
    }

    #[test]
    fn invalid_type_in_create() {
        assert!(CreateQuery::new("create table_name (col1 unknown_type);").is_err())
    }

    #[test]
    fn missing_bracket_in_create() {
        assert!(CreateQuery::new("create table_name (col1 varchar;").is_err())
    }

    #[test]
    fn invalid_separator_in_create() {
        assert!(CreateQuery::new("create table_name (col1 varchar; col2 varchar);").is_err())
    }

    #[test]
    fn redundant_symbols_in_create() {
        assert!(CreateQuery::new("create table_name (col1 varchar) asdasdsasad;").is_err())
    }

    #[test]
    fn wrong_number_of_args_in_col_clause() {
        assert!(CreateQuery::new("create table_name (col1 varchar some_word);").is_err())
    }

    #[test]
    fn case_inconsistent_query() {
        assert!(CreateQuery::new("CReaTE table_name (col1 varchar);").is_ok())
    }

}
