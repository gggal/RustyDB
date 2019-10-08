mod constants;
pub mod create_query;
mod delete_query;
mod drop_query;
mod insert_query;
mod select_query;
mod update_query;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SQLParser;

use self::create_query::CreateQuery;
use self::delete_query::DeleteQuery;
use self::drop_query::DropQuery;
use self::insert_query::InsertQuery;
use self::select_query::SelectQuery;
use self::update_query::UpdateQuery;

extern crate downcast_rs;
use self::downcast_rs::Downcast;

// TODO: optimize this part;
// problem is I can't return result of boxed query, but can return option or box
// itself which is unsafe

/// A factory function that parses a string to internal entity, representing an SQL query.
/// Supported queries are: select, update, delete, create and drop table.
/// Supported variable types are INT, DOUBLE, VARCHAR, NULL.
/// Supported operations are:
///     logical - AND, OR
///     arythmetical - +, -, /, *
/// Supported clauses JOIN, ...
///
pub fn parse(text_query: &str) -> Option<Box<dyn Query>> {
match SQLParser::parse(Rule::QUERY, text_query) {
    
    Ok(boxed) => {
        let a = boxed.next().expect("").as_rule();
        match a {
        Rule::SELECT_QUERY => Some(SelectQuery::new(boxed.next().expect("")).expect("msg: &str")),
        Rule::CREATE_QUERY => Some(CreateQuery::new(a).expect("msg: &str")),
        Rule::INSERT_QUERY => Some(InsertQuery::new(a).expect("msg: &str")),
        Rule::DELETE_QUERY => Some(DeleteQuery::new(a).expect("msg: &str")),
        Rule::UPDATE_QUERY => Some(UpdateQuery::new(a).expect("msg: &str")),
        Rule::DROP_QUERY => Some(DropQuery::new(a).expect("msg: &str"))
        }
    },
    Err(err) => None
}
    // match text_query
    //     .to_string()
    //     .trim()
    //     .split_whitespace()
    //     .next()
    //     .unwrap()
    // {
    //     "insert" => match InsertQuery::new(text_query) {
    //         Ok(boxed_query) => Some(boxed_query),
    //         Err(err_msg) => {
    //             println!(
    //                 "Error occurred while trying to parse {} {}",
    //                 text_query, err_msg
    //             );
    //             None
    //         }
    //     },
    //     "select" => match SelectQuery::new(text_query) {
    //         Ok(boxed_query) => Some(boxed_query),
    //         Err(err_msg) => {
    //             println!(
    //                 "Error occurred while trying to parse {} {}",
    //                 text_query, err_msg
    //             );
    //             None
    //         }
    //     },
    //     "update" => match UpdateQuery::new(text_query) {
    //         Ok(boxed_query) => Some(boxed_query),
    //         Err(err_msg) => {
    //             println!(
    //                 "Error occurred while trying to parse {} {}",
    //                 text_query, err_msg
    //             );
    //             None
    //         }
    //     },
    //     "delete" => match DeleteQuery::new(text_query) {
    //         Ok(boxed_query) => Some(boxed_query),
    //         Err(err_msg) => {
    //             println!(
    //                 "Error occurred while trying to parse {} {}",
    //                 text_query, err_msg
    //             );
    //             None
    //         }
    //     },
    //     "create" => match CreateQuery::new(text_query) {
    //         Ok(boxed_query) => Some(boxed_query),
    //         Err(err_msg) => {
    //             println!(
    //                 "Error occurred while trying to parse {} {}",
    //                 text_query, err_msg
    //             );
    //             None
    //         }
    //     },
    //     "drop" => match DropQuery::new(text_query) {
    //         Ok(boxed_query) => Some(boxed_query),
    //         Err(err_msg) => {
    //             println!(
    //                 "Error occurred while trying to parse {} {}",
    //                 text_query, err_msg
    //             );
    //             None
    //         }
    //     },
    //     _ => {
    //         println!(
    //             "Invalid command received {}! Find the supported syntax here ... .",
    //             text_query
    //         );
    //         None
    //     }
    // }
}

pub trait Query: Downcast {
    fn validate(&self) -> String;
    // fn build_plan(&self, stream : Iterator<&u64> ) -> Iterator<&u64>;
}

downcast_rs::impl_downcast!(Query);

impl Query for InsertQuery {
    fn validate(&self) -> String {
        String::from("")
    }

    // fn build_plan(&self) -> String {
    //     String::from("")
    // }
}

impl Query for UpdateQuery {
    fn validate(&self) -> String {
        String::from("")
    }

    // fn build_plan(self, it : Iterator) -> Iterator {
    //     it.filter(self.FilterTree.evaluate())
    //     .map(|r| {r.replace(self.tuples)})
    // }
}

impl Query for DeleteQuery {
    fn validate(&self) -> String {
        String::from("")
    }

    // fn build_plan(self, it : Iterator) -> Iterator {
    //     it.filter(self.FilterTree.evaluate())
    // }
}

impl Query for CreateQuery {
    fn validate(&self) -> String {
        String::from("")
    }

    // fn build_plan(&self) -> String {
    //     String::from("")
    // }
}

impl Query for DropQuery {
    fn validate(&self) -> String {
        String::from("")
    }

    // fn build_plan(&self) -> String {
    //     String::from("")
    // }
}

// struct Column {
//     table_name : String,
//     col_name : String,
//     alias : String
// }

// impl Column {
//     fn new(text: String) -> Result<Column, String> {
//         let regex = Regex::new(r"().() as ()").unwrap();
//         let cap = regex.captures_iter(&text).next();
//         match regex.captures_iter(&text).next() {
//             None => Err(String::from("")),
//             Some(cap) => Ok(Column{table_name: String::from(&cap[1]),col_name: String::from(&cap[2]),alias: String::from(&cap[3])})
//         }
//     }
// }

impl Query for SelectQuery {
    fn validate(&self) -> String {
        String::from("")
    }

    // fn build_plan(self, it : Iterator) -> Iterator {
    //     it.filter(self.FilterTree.evaluate())
    // }
}
