use regex::Regex;

pub struct DropQuery {
    table: String,
}

impl DropQuery {
    pub fn new(text: String) -> Result<Box<DropQuery>, String> {
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
            _ => Err(String::from("")),
        }
    }
}
