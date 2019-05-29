// mod filter;
use filter::filter_tree;
//use futures::stream;
// pub mod parser {

use regex::Regex;

/// A factory function that parses a string to internal entity, representing an SQL query. 
/// Supported queries are: select, update, delete, create and drop table.
/// Supported variable types are INT, DOUBLE, VARCHAR, NULL. 
/// Supported operations are:
///     logical - AND, OR
///     arythmetical - +, -, /, *
/// Supported clauses JOIN, ...
/// 
pub fn parse(text_query : &str) -> Box<Query> {
    match text_query.to_string().trim().split_whitespace().next().unwrap() {
        "insert" => InsertQuery::new(text_query.to_string()),

        "select" => SelectQuery::new(text_query.to_string()),
        "update" => UpdateQuery::new(text_query.to_string()),
        "delete" => DeleteQuery::new(text_query.to_string()),
        "create" => Box::new(CreateTableQuery::new(text_query.to_string())),
        "drop" => Box::new(DropTableQuery::new(text_query.to_string())),
         _ => Err(String::from("Invalid command."))
    }
}

pub fn parse_valid(text_query : &str) -> Box<Query> {
    let a =
    match text_query.to_string().trim().split_whitespace().next().unwrap() {
        "insert" => InsertQuery::new(text_query.to_string()),

        "select" => SelectQuery::new(text_query.to_string()),
        "update" => UpdateQuery::new(text_query.to_string()),
        "delete" => DeleteQuery::new(text_query.to_string()),
        "create" => Box::new(CreateTableQuery::new(text_query.to_string())),
        "drop" => Box::new(DropTableQuery::new(text_query.to_string())),
         _ => Err(String::from("Invalid command."))
    };
}

pub fn asd() -> Box<Query> {
    match 5 % 2 {
        1 => InsertQuery::new(String::from("")).unwrap(),
        _ => SelectQuery::new(String::from("")).unwrap()
    }
}

pub fn asd1() -> Result<Box<Query>, String> {
    match 5 % 2 {
        1 => InsertQuery::new(String::from("")),
        _ => SelectQuery::new(String::from(""))
    }
}

//todo var
enum Type {
    INT(i64),
    DOUBLE(f64),
    VARCHAR(String),
    NULL
}


trait Query {
    fn validate(&self) -> String;
    // fn build_plan(&self, stream : Iterator<&u64> ) -> Iterator<&u64>;
}

struct InsertQuery {
    columns : Vec<String>,
    values : Vec<Type>,
    table_name : String
}

impl InsertQuery {
 fn new(text : String) -> Result<Box<InsertQuery>, String> {
    let regex = Regex::new(r"
        (i?:insert[[:space:]]+into[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+\((P<names>?[[:alnum:]]*)\)[[:space:]]+values[[:space:]]+(P<values>[[:space:]]+)").unwrap();
        let mut res : Result<Box<InsertQuery>, String> = Err(String::from("failed"));

        for cap in regex.captures_iter(&text) {
            let table_name = String::from(&cap[1]);
            let regex = Regex::new(r"(table_name//.).{0,1}[  a-Z_].{1}[a-Z1-9_].*").unwrap();
            let mut values : Vec<Type> = Vec::new();
            if cap[2].split(',').all(|col| {regex.is_match(col)}) {
                //Err(());
            }   
            // for val in cap[3].split(',') {
            //     match str_to_type(val) {
            //         Ok(t) => values.insert(t),
            //         Err(err) => 
            //     }
            // }

            let columns : Vec<String> = cap[2].split(',')
                    .map(|col| {String::from(col)}).collect(); //map all to trim func
            // let values : Vec<Type> = cap[3].split(',')
            //         .map(|val| {}).collect();

            // let res : Result<InsertQuery, ()>;
            if columns.len() != values.len() {
                res = Err(String::from("failed"));
            } else {
                res = Ok(Box::new(InsertQuery{table_name, columns, values}));
            }
        }
        res
    }
}

impl Query for InsertQuery {
    fn validate(&self) -> String {

        String::from("")        
    }

    // fn build_plan(&self) -> String {
    //     String::from("")
    // }
}

struct UpdateQuery {
    filter : filter_tree,
    tuples : Vec<(String, String)>,
    table : String
}

impl UpdateQuery {
    fn new(text : &str) -> Result<Box<UpdateQuery>, String> {
        let regex = Regex::new(r"
        (i?:update[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+set(P<set>?([[:alnum:]]+=.*)+)where[[:space:]]+(P<tree>.*)[[:space:]]*&").unwrap();

        match regex.captures(text) {
            None => Err(String::from("")),
            Some(matched) => 
                match (matched.name("table"), matched.name("set"), matched.name("tree")) {
                    (Some(table), Some(set), Some(tree)) => {
                       let tupled : Vec<(String, String)> = set.as_str().split(",").map(|a| {(a.split("=").next().unwrap().trim().to_string(), a.split("=").next().unwrap().trim().to_string())}).collect(); 
                        match filter_tree::new(tree.as_str()) {
                            Err(_) => Err(String::from("")),
                            Ok(filter) => 
                                
                                Ok(
                                Box::new(
                                UpdateQuery{
                                    table: String::from(table.as_str()),
                                    tuples: tupled,
                                    filter})
                                    )
                        }
                        
                    }
                    _ => Err(String::from(""))
                } 
        }
    }
}

impl Query for UpdateQuery {
    fn validate(&self) -> String {
        String::from("")        
    }

    // fn build_plan(self, it : Iterator) -> Iterator {
    //     it.filter(self.filter_tree.evaluate())
    //     .map(|r| {r.replace(self.tuples)})
    // }
} 

struct DeleteQuery {
    filter : filter_tree,
    table : String
}

impl DeleteQuery {
    fn new(text : String) -> Result<Box<DeleteQuery>, String> {
        let regex = Regex::new(r"
        (i?:delete[[:space:]]+from[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+where[[:space:]]+(P<tree>.*)[[:space:]]*&").unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),
            Some(matched) => 
                match (matched.name("table"), matched.name("tree")) {
                    (Some(table), Some(tree)) => {
                        match filter_tree::new(tree.as_str()) {
                            Err(_) => Err(String::from("")),
                            Ok(filter) => Ok(
                                Box::new(
                                DeleteQuery{
                                    table: String::from(table.as_str()),
                                    filter})
                            )
                        }
                    }
                    _ => Err(String::from(""))
                } 
        }
    }
}

impl Query for DeleteQuery {
    fn validate(&self) -> String {
        String::from("")        
    }

    // fn build_plan(self, it : Iterator) -> Iterator {
    //     it.filter(self.filter_tree.evaluate())
    // }
} 


struct CreateQuery {
    table : String,
    columns: Vec<(String, String)>
}

impl CreateQuery {
    fn new(text : String) -> Result<Box<CreateQuery>, String> {
        let regex = Regex::new(r"
        (i?:create[[:space:]]+table[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+\((P<cols>([[:space:]]*.*[[:space:]]+(varchar|int|float)[[:space:]]*,)*([[:space:]]*.*[[:space:]]+(varchar|int|float)[[:space:]]*))\)[[:space:]]*&").unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),
            Some(matched) => 
                match (matched.name("table"), matched.name("cols")) {
                    (Some(table), Some(cols)) => {
                        Ok(Box::new(CreateQuery{
                            table: String::from(table.as_str()),
                            columns: cols.as_str().split(",").map(|a| { (String::from(a.split_whitespace().next().unwrap()), String::from(a.split_whitespace().last().unwrap()))}).collect()
                            })
                        )
                    }
                    _ => Err(String::from(""))
                }
                    _ => Err(String::from(""))
        }
    }
}

impl Query for CreateQuery {
    fn validate(&self) -> String {
        String::from("")        
    }

    // fn build_plan(&self) -> String {
    //     String::from("")
    // }
} 

struct DropQuery {
    table : String
}

impl DropQuery {
    fn new(text : String) -> Result<Box<DropQuery>, String> {
        let regex = Regex::new(r"
        (i?:drop[[:space:]]+table[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]*&").unwrap();

        match regex.captures(&text) {
            None => Err(String::from("")),
            Some(matched) => 
                match matched.name("table") {
                    Some(table) => {
                        Ok(Box::new(DropQuery{table: String::from(table.as_str())})
                        )
                    }
                    _ => Err(String::from(""))
                }
                    _ => Err(String::from(""))
        }
    }
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

struct SelectQuery {
    cols : Vec<(Option<String>, String, Option<String>)>,
    from : (Result<String, Box<SelectQuery>>, Option<String>),
    filter : Option<filter_tree>,
    // joins : Vec<Box<SelectQuery>>
}

impl SelectQuery {

    fn new(text: String) -> Result<Box<SelectQuery>, String> {
        //match text.match("^select * from table where *"&)
        let regex = Regex::new(r"^(?i)select[[:space:]]+(?P<cols>([[:alnum:]]*\.)?[[:alnum:]]+[[:space:]]+(as[[:space:]]+[[:alnum:]]+)?)[[:space:]]*from[[:space:]]+((?P<sub>\(.*\))|(?P<table>a))[[:space:]]+(where[[:space:]]+(?P<tree>.*))?$").unwrap();
        let res : Result<SelectQuery, ()>;
        
        match regex.captures(&text) {
            None => Err(String::from("")),
                
            Some(cap) => {
                match (cap.name("cols"), cap.name("table"), cap.name("filter")) {
                    //if either 'select' or 'from' clause is missing the query is not valid
                    (Some(columns), Some(table), None) =>
                        match (SelectQuery::parse_select_cols(String::from(columns.as_str())), SelectQuery::parse_select_from(table.as_str().to_string())) {
                            (Ok(c), Some(s)) => Ok(Box::new(SelectQuery{cols: c, from: s, filter: None})),
                            _ => Err(String::from(""))
                        }
                    ,
                    (Some(columns), Some(table), Some(tree)) =>
                        match (SelectQuery::parse_select_cols(String::from(columns.as_str())), SelectQuery::parse_select_from(table.as_str().to_string()), filter_tree::new(&tree.as_str())) {
                            (Ok(c), Some(s), Ok(t)) => Ok(Box::new(SelectQuery{cols: c, from: s, filter: Some(t)})),
                            _ => Err(String::from(""))
                        }
                    ,
                    _ => Err("".to_string())
                        
                }
            }
        } 
        
    }

    fn parse_select_cols(text : String) -> Result<Vec<(Option<String>, String, Option<String>)>, String> {
        let regex = Regex::new(r"((P<table_name>[[:alnum:]]+)\.)?(P<name>[[:alnum:]]+)(iP<alias>?[[:space:]]+as[[:space:]]+[[:alnum:]])").unwrap();
        let mut res : Vec<(Option<String>, String, Option<String>)> = Vec::new();
        let mut v : bool = true;
        for col in text.split(",") {
            match regex.captures(col.trim()) {
                None => v = false,
                Some(matched) => 
                    match (matched.name("table_name"), matched.name("name"), matched.name("alias")) {
                        (_,None,_) => v = false,
                        (Some(a), Some(b), Some(c)) => { 
                            res.push((Some(String::from(a.as_str())), String::from(b.as_str()), Some(String::from(c.as_str()))));
                             },
                        (None, Some(a), Some(b)) => { 
                            res.push((None, String::from(a.as_str()), Some(String::from(a.as_str()))));
                             },
                        (Some(a), Some(b), None) => { 
                            res.push((Some(String::from(a.as_str())), String::from(b.as_str()), Some(String::from(a.as_str()))));
                             },
                        (None, Some(a), None) => {
                            res.push((None, String::from(a.as_str()), None));
                            }
                    }
            }
        }

        if v == true {
            Ok(res)
        } else {
        Err(String::from(""))
        }
    }

    fn parse_select_from(text : String) -> Option<(Result<String, Box<SelectQuery>>, Option<String>)> {
        let regex = Regex::new(r"^(i?(\(P<sub>[[:alnum:]]+\))[[:space:]]+(as[[:space:]]+)*(P<alias>?[[:alnum:]]+)[[:space:]]*&)").unwrap();
        match regex.captures(&text) {
            Some(matched) => 
                match (matched.name("sub"), matched.name("alias")) {
                    (None, _) => None,
                    (Some(sub), None) => 
                        match SelectQuery::new(sub.as_str().to_string()) {
                            Err(a) => None,
                            Ok(s) => Some((Err(s), None))
                        }
                    ,
                    (Some(sub), Some(al)) => 
                        match SelectQuery::new(sub.as_str().to_string()) {
                            Err(a) => None,
                            Ok(s) => Some((Err(s), Some(al.as_str().to_string())))
                        } 
                }
            None => None
        }
    }
}

impl Query for SelectQuery {
    fn validate(&self) -> String {
        String::from("")        
    }

    // fn build_plan(self, it : Iterator) -> Iterator {
    //     it.filter(self.filter_tree.evaluate())
    // }
} 