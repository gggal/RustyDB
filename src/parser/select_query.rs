use filter::FilterTree;
use regex::Regex;

pub struct SelectQuery {
    cols: Vec<(Option<String>, String, Option<String>)>,
    from: (Result<String, Box<SelectQuery>>, Option<String>),
    filter: Option<FilterTree>,
    // joins : Vec<Box<SelectQuery>>
}

impl SelectQuery {
    pub fn new(text: String) -> Result<Box<SelectQuery>, String> {
        //match text.match("^select * from table where *"&)
        let regex = Regex::new(r"^(?i)select[[:space:]]+(?P<cols>([[:alnum:]]*\.)?[[:alnum:]]+[[:space:]]+(as[[:space:]]+[[:alnum:]]+)?)[[:space:]]*from[[:space:]]+((?P<sub>\(.*\))|(?P<table>a))[[:space:]]+(where[[:space:]]+(?P<tree>.*))?$").unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),

            Some(cap) => {
                match (cap.name("cols"), cap.name("table"), cap.name("filter")) {
                    //if either 'select' or 'from' clause is missing the query is not valid
                    (Some(columns), Some(table), None) => match (
                        SelectQuery::parse_select_cols(String::from(columns.as_str())),
                        SelectQuery::parse_select_from(table.as_str().to_string()),
                    ) {
                        (Ok(c), Some(s)) => Ok(Box::new(SelectQuery {
                            cols: c,
                            from: s,
                            filter: None,
                        })),
                        _ => Err(String::from("")),
                    },
                    (Some(columns), Some(table), Some(tree)) => match (
                        SelectQuery::parse_select_cols(String::from(columns.as_str())),
                        SelectQuery::parse_select_from(table.as_str().to_string()),
                        FilterTree::new(&tree.as_str()),
                    ) {
                        (Ok(c), Some(s), Ok(t)) => Ok(Box::new(SelectQuery {
                            cols: c,
                            from: s,
                            filter: Some(t),
                        })),
                        _ => Err(String::from("")),
                    },
                    _ => Err("".to_string()),
                }
            }
        }
    }
    fn parse_select_cols(
        text: String,
    ) -> Result<Vec<(Option<String>, String, Option<String>)>, String> {
        let regex = Regex::new(r"((P<table_name>[[:alnum:]]+)\.)?(P<name>[[:alnum:]]+)(iP<alias>?[[:space:]]+as[[:space:]]+[[:alnum:]])").unwrap();
        let mut res: Vec<(Option<String>, String, Option<String>)> = Vec::new();
        let mut v: bool = true;
        for col in text.split(",") {
            match regex.captures(col.trim()) {
                None => v = false,
                Some(matched) => match (
                    matched.name("table_name"),
                    matched.name("name"),
                    matched.name("alias"),
                ) {
                    (_, None, _) => v = false,
                    (Some(a), Some(b), Some(c)) => {
                        res.push((
                            Some(String::from(a.as_str())),
                            String::from(b.as_str()),
                            Some(String::from(c.as_str())),
                        ));
                    }
                    (None, Some(a), Some(b)) => {
                        res.push((
                            None,
                            String::from(a.as_str()),
                            Some(String::from(a.as_str())),
                        ));
                    }
                    (Some(a), Some(b), None) => {
                        res.push((
                            Some(String::from(a.as_str())),
                            String::from(b.as_str()),
                            Some(String::from(a.as_str())),
                        ));
                    }
                    (None, Some(a), None) => {
                        res.push((None, String::from(a.as_str()), None));
                    }
                },
            }
        }

        if v == true {
            Ok(res)
        } else {
            Err(String::from(""))
        }
    }

    fn parse_select_from(
        text: String,
    ) -> Option<(Result<String, Box<SelectQuery>>, Option<String>)> {
        let regex = Regex::new(r"^(i?(\(P<sub>[[:alnum:]]+\))[[:space:]]+(as[[:space:]]+)*(P<alias>?[[:alnum:]]+)[[:space:]]*&)").unwrap();
        match regex.captures(&text) {
            Some(matched) => match (matched.name("sub"), matched.name("alias")) {
                (None, _) => None,
                (Some(sub), None) => match SelectQuery::new(sub.as_str().to_string()) {
                    Err(a) => None,
                    Ok(s) => Some((Err(s), None)),
                },
                (Some(sub), Some(al)) => match SelectQuery::new(sub.as_str().to_string()) {
                    Err(a) => None,
                    Ok(s) => Some((Err(s), Some(al.as_str().to_string()))),
                },
            },
            None => None,
        }
    }
}
