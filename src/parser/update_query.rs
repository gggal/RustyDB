use filter::FilterTree;
use regex::Regex;

pub struct UpdateQuery {
    filter: FilterTree,
    tuples: Vec<(String, String)>,
    table: String,
}

impl UpdateQuery {
    pub fn new(text: &str) -> Result<Box<UpdateQuery>, String> {
        let regex = Regex::new(r"
        (i?:update[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+set(P<set>?([[:alnum:]]+=.*)+)where[[:space:]]+(P<tree>.*)[[:space:]]*&").unwrap();

        match regex.captures(text) {
            None => Err(String::from("")),
            Some(matched) => match (
                matched.name("table"),
                matched.name("set"),
                matched.name("tree"),
            ) {
                (Some(table), Some(set), Some(tree)) => {
                    let tupled: Vec<(String, String)> = set
                        .as_str()
                        .split(",")
                        .map(|a| {
                            (
                                a.split("=").next().unwrap().trim().to_string(),
                                a.split("=").next().unwrap().trim().to_string(),
                            )
                        })
                        .collect();
                    match FilterTree::new(tree.as_str()) {
                        Err(_) => Err(String::from("")),
                        Ok(filter) => Ok(Box::new(UpdateQuery {
                            table: String::from(table.as_str()),
                            tuples: tupled,
                            filter,
                        })),
                    }
                }
                _ => Err(String::from("")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::UpdateQuery;

    #[test]
    fn missing_set_clause() {
        assert!(UpdateQuery::new("update 'a_table' arg1=1;").is_err())
    }

    #[test]
    fn invalid_separator_in_set_clause() {
        assert!(UpdateQuery::new("update 'a_table' set arg1=1 and arg2=2;").is_err())
    }
}
