use regex::Regex;

#[derive(Debug)]
pub struct CreateQuery {
    table: String,
    columns: Vec<(String, String)>,
}

impl CreateQuery {
    pub fn new(text: String) -> Result<Box<CreateQuery>, String> {
        let regex = Regex::new(r"
        (i?:create[[:space:]]+table[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+\((P<cols>([[:space:]]*.*[[:space:]]+(varchar|int|float)[[:space:]]*,)*([[:space:]]*.*[[:space:]]+(varchar|int|float)[[:space:]]*))\)[[:space:]]*&").unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),
            Some(matched) => match (matched.name("table"), matched.name("cols")) {
                (Some(table), Some(cols)) => Ok(Box::new(CreateQuery {
                    table: String::from(table.as_str()),
                    columns: cols
                        .as_str()
                        .split(",")
                        .map(|a| {
                            (
                                String::from(a.split_whitespace().next().unwrap()),
                                String::from(a.split_whitespace().last().unwrap()),
                            )
                        })
                        .collect(),
                })),
                _ => Err(String::from("")),
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use parser::CreateQuery;

    #[test]
    fn missing_table_clause() {
        assert_eq!(CreateQuery::new(String::from("create table_name;")).is_err(), true)
    }
}