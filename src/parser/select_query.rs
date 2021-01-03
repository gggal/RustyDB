// use filter::FilterTree;
// use regex::Regex;

// use parser::Rule;
// use pest::iterators::Pair;

// use std::collections::HashSet;

// use std::hash::{Hash, Hasher};

// #[derive(Eq)]
// struct SelectColumn {
//     table: String,
//     name: String,
//     alias: String
// }

// #[derive(Eq)]
// pub struct SelectQuery {
//     cols: Vec<(SelectColumn)>,
//     from: (String, Box<SelectQuery>),
//     filter: Option<FilterTree>,
//     // joins : Vec<Box<SelectQuery>>
// }

// impl PartialEq for SelectColumn {
//     fn eq(&self, other: &Self) -> bool {
//         self.table == other.table &&self.name == other.name
//     }
// }

// impl PartialEq for SelectQuery {
//     fn eq(&self, other: &Self) -> bool {
//         true
//     }
// }

// trait Eq: PartialEq<Self> {}

// impl Hash for SelectColumn {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.table.hash(state);
//         self.name.hash(state);
//     }
// }

// impl SelectQuery {
//     pub fn new(rule: Pair<Rule>) -> Result<Box<SelectQuery>, &str> {

//         let iter = rule.into_inner();        

//         //assuming select query has 5 clauses => select, from, where, group by, having (eventually can add order by etc)

//         // working on the select clause of the query
//         match parse_select_clause(iter.next().unwrap().as_rule()) {
//             None => "",
//             Ok(HashSet<>)
//         }

//         // working on the select clause of the query
//         if let Rule::SELECT_CLAUSE = iter.next().as_rule {
//             //in the select clause account for every select col:
//             for col in iter.next.as_inner() {
//                 match col.as_inner {
//                     "*" => "Select all",
//                     Rule::SelectColumn => add to a map,
//                     unreachable!("A column is neighter an asterix nor a column token")
//                 }
//             }
//         } else {
//             unreachable!("Query doesn't conform to the standard: no select clause in select query")
//         }

//         while innerRule in rule.into_inner() {
//             match innerRule {
//                 "asline.into_innerd" => "",
//             };
//         }
//         Err("")
//     }

//     fn parse_select_clause(&mut self, rule: Pair<Rule>) -> Option<HashSet<SelectColumn>> {
//         if let Rule::SELECT_CLAUSE = rule {

//             let res = HashSet::new();
//             let all : bool = false;
//             //in the select clause account for every select col:
//             for col in rule.as_inner() {
//                 match col.as_inner {
//                     "*" => all = true,
//                     Rule::SELECT_COLUMN => { res.insert(col.as_inner); },
//                     _ => unreachable!("A column is neighter an asterix nor a column token")
//                 }
//             }
//             Some(res)
//         } else {
//             None
//         }
//     }


//     fn parse_select_cols(
//         text: String,
//     ) -> Result<Vec<(Option<String>, String, Option<String>)>, String> {
//         let regex = Regex::new(r"((P<table_name>[[:alnum:]]+)\.)?(P<name>[[:alnum:]]+)(iP<alias>?[[:space:]]+as[[:space:]]+[[:alnum:]])").unwrap();
//         let mut res: Vec<(Option<String>, String, Option<String>)> = Vec::new();
//         let mut v: bool = true;
//         for col in text.split(",") {
//             match regex.captures(col.trim()) {
//                 None => v = false,
//                 Some(matched) => match (
//                     matched.name("table_name"),
//                     matched.name("name"),
//                     matched.name("alias"),
//                 ) {
//                     (_, None, _) => v = false,
//                     (Some(a), Some(b), Some(c)) => {
//                         res.push((
//                             Some(String::from(a.as_str())),
//                             String::from(b.as_str()),
//                             Some(String::from(c.as_str())),
//                         ));
//                     }
//                     (None, Some(a), Some(b)) => {
//                         res.push((
//                             None,
//                             String::from(a.as_str()),
//                             Some(String::from(a.as_str())),
//                         ));
//                     }
//                     (Some(a), Some(b), None) => {
//                         res.push((
//                             Some(String::from(a.as_str())),
//                             String::from(b.as_str()),
//                             Some(String::from(a.as_str())),
//                         ));
//                     }
//                     (None, Some(a), None) => {
//                         res.push((None, String::from(a.as_str()), None));
//                     }
//                 },
//             }
//         }

//         if v == true {
//             Ok(res)
//         } else {
//             Err(String::from(""))
//         }
//     }

//     fn parse_select_from(
//         text: String,
//     ) -> Option<(Result<String, Box<SelectQuery>>, Option<String>)> {
//         let regex = Regex::new(r"^(i?(\(P<sub>[[:alnum:]]+\))[[:space:]]+(as[[:space:]]+)*(P<alias>?[[:alnum:]]+)[[:space:]]*&)").unwrap();
//         match regex.captures(&text) {
//             Some(matched) => match (matched.name("sub"), matched.name("alias")) {
//                 (None, _) => None,
//                 (Some(sub), None) => match SelectQuery::new(sub.as_str()) {
//                     Err(_) => None,
//                     Ok(s) => Some((Err(s), None)),
//                 },
//                 (Some(sub), Some(al)) => match SelectQuery::new(sub.as_str()) {
//                     Err(_) => None,
//                     Ok(s) => Some((Err(s), Some(al.as_str().to_string()))),
//                 },
//             },
//             None => None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use parser::SelectQuery;

//     #[test]
//     fn omitted_alias_select() {
//         assert!(SelectQuery::new("select a.arg1 from 'a_table' a;").is_ok())
//     }

//     #[test]
//     fn unknown_operator_in_where_clause() {
//         assert!(SelectQuery::new("select arg1 from 'a_table' where arg1 ? 2;").is_err())
//     }

//     #[test]
//     fn corrupt_filter_tree_in_where_clause_1() {
//         assert!(SelectQuery::new("select arg1 from 'a_table' where arg1 + 2;").is_err())
//     }

//     #[test]
//     fn corrupt_filter_tree_in_where_clause_2() {
//         assert!(SelectQuery::new("select arg1 from 'a_table' where 1 < 2 and arg1 + 2;").is_err())
//     }

//     #[test]
//     fn empty_where_clause() {
//         assert!(SelectQuery::new("select 123 from 'a_table' where;").is_err())
//     }

//     #[test]
//     fn missing_where_clause() {
//         assert!(SelectQuery::new("select 123 from 'a_table';").is_ok())
//     }

//     #[test]
//     fn missing_on_in_join_clause() {
//         assert!(SelectQuery::new("select a.arg1 from 'a_table' a join 'b_table' b;").is_err())
//     }

//     #[test]
//     fn ommitting_alias_in_join_clause() {
//         assert!(
//             SelectQuery::new("select a.arg1 from 'a_table' a join 'b_table' b on arg1;").is_err()
//         )
//     }
// }