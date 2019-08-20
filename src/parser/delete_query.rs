use filter::FilterTree;
use regex::Regex;

pub struct DeleteQuery {
    filter: FilterTree,
    table: String,
}

impl DeleteQuery {
    pub fn new(text: String) -> Result<Box<DeleteQuery>, String> {
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
