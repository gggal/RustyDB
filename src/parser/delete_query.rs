use filter::FilterTree;
use parser::constants::DELETE_QUERY_REGEX;

pub struct DeleteQuery {
    filter: FilterTree,
    pub table: String,
}

impl DeleteQuery {
    pub fn new(text: &str) -> Result<Box<DeleteQuery>, &'static str> {
        match DELETE_QUERY_REGEX.captures(&text) {
            None => Err("Given query doesn't conform to the supported syntax."),
            Some(matched) => match (matched.name("table"), matched.name("tree")) {
                (Some(table), Some(tree)) => match FilterTree::new(tree.as_str()) {
                    Err(_) => Err(
                        "Where clause for the given query doesn't conform to the supported syntax.",
                    ),
                    Ok(filter) => Ok(Box::new(DeleteQuery {
                        table: String::from(table.as_str()),
                        filter,
                    })),
                },
                _ => Err("Given query doesn't conform to the supported syntax."),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::DeleteQuery;

    #[test]
    #[ignore]
    fn ommitted_where_clause() {
        assert!(DeleteQuery::new("delete from 'a_table';").is_ok());
    }

    #[test]
    fn missing_from_word_in_delete_statement() {
        assert!(DeleteQuery::new("delete 'a_table';").is_err());
    }

    #[test]
    fn correct_table_name_is_accuired() {
        let query = DeleteQuery::new("delete a_table where id = 2;").unwrap();
        assert_eq!(query.table, "a_table");
    }
}
