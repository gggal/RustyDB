use filter::FilterTree;
use regex::Regex;

use parser::Rule;
use pest::iterators::Pair;

struct SelectColumn {
    table: String,
    name: String,
    alias: String
}

pub struct SelectQuery {
    cols: Vec<(SelectColumn)>,
    from: (Result<String, Box<SelectQuery>>, Option<String>),
    filter: Option<FilterTree>,
    // joins : Vec<Box<SelectQuery>>
}

impl SelectQuery {
    pub fn new(rule: Pair<Rule>) -> Result<Box<SelectQuery>, &str> {

        let iter = rule.into_inner();

        let first : bool = true;
        //select clause
        for col in iter {
            if let Rule::SELECT_COLUMN = col.as_rule() {
                
            } else {
                if first {
                    Err("Select clause in empty")
                }
                break;
            }



        }

        while innerRule in rule.into_inner() {
            match innerRule {
                "asline.into_innerd" => "",
            };
        }
        Err("")

        // match regex.captures(&text) {
        //     None => Err(String::from("")),

        //     Some(cap) => {
        //         match (cap.name("cols"), cap.name("table"), cap.name("filter")) {
        //             //if either 'select' or 'from' clause is missing the query is not valid
        //             (Some(columns), Some(table), None) => match (
        //                 SelectQuery::parse_select_cols(String::from(columns.as_str())),
        //                 SelectQuery::parse_select_from(table.as_str().to_string()),
        //             ) {
        //                 (Ok(c), Some(s)) => Ok(Box::new(SelectQuery {
        //                     cols: c,
        //                     from: s,
        //                     filter: None,
        //                 })),
        //                 _ => Err(String::from("")),
        //             },
        //             (Some(columns), Some(table), Some(tree)) => match (
        //                 SelectQuery::parse_select_cols(String::from(columns.as_str())),
        //                 SelectQuery::parse_select_from(table.as_str().to_string()),
        //                 FilterTree::new(&tree.as_str()),
        //             ) {
        //                 (Ok(c), Some(s), Ok(t)) => Ok(Box::new(SelectQuery {
        //                     cols: c,
        //                     from: s,
        //                     filter: Some(t),
        //                 })),
        //                 _ => Err(String::from("")),
        //             },
        //             _ => Err("".to_string()),
        //         }
        //     }
        // }
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
                (Some(sub), None) => match SelectQuery::new(sub.as_str()) {
                    Err(_) => None,
                    Ok(s) => Some((Err(s), None)),
                },
                (Some(sub), Some(al)) => match SelectQuery::new(sub.as_str()) {
                    Err(_) => None,
                    Ok(s) => Some((Err(s), Some(al.as_str().to_string()))),
                },
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::SelectQuery;

    #[test]
    fn omitted_alias_select() {
        assert!(SelectQuery::new("select a.arg1 from 'a_table' a;").is_ok())
    }

    #[test]
    fn unknown_operator_in_where_clause() {
        assert!(SelectQuery::new("select arg1 from 'a_table' where arg1 ? 2;").is_err())
    }

    #[test]
    fn corrupt_filter_tree_in_where_clause_1() {
        assert!(SelectQuery::new("select arg1 from 'a_table' where arg1 + 2;").is_err())
    }

    #[test]
    fn corrupt_filter_tree_in_where_clause_2() {
        assert!(SelectQuery::new("select arg1 from 'a_table' where 1 < 2 and arg1 + 2;").is_err())
    }

    #[test]
    fn empty_where_clause() {
        assert!(SelectQuery::new("select 123 from 'a_table' where;").is_err())
    }

    #[test]
    fn missing_where_clause() {
        assert!(SelectQuery::new("select 123 from 'a_table';").is_ok())
    }

    #[test]
    fn missing_on_in_join_clause() {
        assert!(SelectQuery::new("select a.arg1 from 'a_table' a join 'b_table' b;").is_err())
    }

    #[test]
    fn ommitting_alias_in_join_clause() {
        assert!(
            SelectQuery::new("select a.arg1 from 'a_table' a join 'b_table' b on arg1;").is_err()
        )
    }
}
