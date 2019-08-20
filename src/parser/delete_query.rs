use filter::FilterTree;
use regex::Regex;

pub struct DeleteQuery {
    filter: FilterTree,
    table: String,
}

impl DeleteQuery {
    pub fn new(text: &str) -> Result<Box<DeleteQuery>, String> {
        let regex = Regex::new(r"
        (i?:delete[[:space:]]+from[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+where[[:space:]]+(P<tree>.*)[[:space:]]*&").unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),
            Some(matched) => match (matched.name("table"), matched.name("tree")) {
                (Some(table), Some(tree)) => match FilterTree::new(tree.as_str()) {
                    Err(_) => Err(String::from("")),
                    Ok(filter) => Ok(Box::new(DeleteQuery {
                        table: String::from(table.as_str()),
                        filter,
                    })),
                },
                _ => Err(String::from("")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::DeleteQuery;

    #[test]
    fn ommitted_where_clause() {
        assert!(DeleteQuery::new("delete from 'a_table';").is_ok());
    }

    #[test]
    fn missing_from_word_in_delete_statement() {
        assert!(DeleteQuery::new("delete 'a_table';").is_err());
    }
}
