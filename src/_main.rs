#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate regex;

pub mod filter;
pub mod logical_operation;
pub mod parser;
pub mod record;

fn main() {
    // //parser::parse("hello");
    // //r"(?P<y>\d{4})"
    // let re = regex::Regex::new(
    // r"^(?i)select[[:space:]]+(?P<hey>([[:alnum:]]*\.)?[[:alnum:]]+[[:space:]]+(as[[:space:]]+[[:alnum:]]+)?)[[:space:]]*from[[:space:]]+((?P<sub>\(.*\))|(?P<table>a))[[:space:]]+(where[[:space:]]+(?P<tree>.*))?$").unwrap();
    //     // [[:space:]]+from[[:space:]]+((?P<sub>\(.*\))|(?P<table>a))$").unwrap();
    //     // (?P<joins>((right|left|inner|outer)? join .* on .*=.*)*)
    //     // where(?P<tree>.*)$").unwrap();
    // let before = "SELECT 12 from (a) ";
    // // let after = re.replace_all(before, "$hey");
    // let cap : regex::Captures = re.captures(before).unwrap();
    // let group = cap.name("hey").unwrap();
    // // let table = cap.name("sub").unwrap();
    // println!("columns => {}", group.as_str());
    // // println!("subtable => {}", table.as_str());
    // //assert_eq!(after, "//2012-03-14");
}
