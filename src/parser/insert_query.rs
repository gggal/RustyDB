use filter::Type;
use regex::Regex;

// #[derive(Debug)]
pub struct InsertQuery {
    columns: Vec<String>,
    values: Vec<Type>,
    table_name: String,
}

impl InsertQuery {
    pub fn new(text: String) -> Result<Box<InsertQuery>, String> {
        let regex = Regex::new(r"
        (i?:insert[[:space:]]+into[[:space:]]+(P<table>?[[:alnum:]]*)[[:space:]]+\((P<names>?[[:alnum:]]*)\)[[:space:]]+values[[:space:]]+(P<values>[[:space:]]+)").unwrap();
        let mut res: Result<Box<InsertQuery>, String> = Err(String::from("failed"));

        for cap in regex.captures_iter(&text) {
            let table_name = String::from(&cap[1]);
            let regex = Regex::new(r"(table_name//.).{0,1}[  a-Z_].{1}[a-Z1-9_].*").unwrap();
            let values: Vec<Type> = Vec::new();
            if cap[2].split(',').all(|col| regex.is_match(col)) {
                //Err(());
            }
            // for val in cap[3].split(',') {
            //     match str_to_type(val) {
            //         Ok(t) => values.insert(t),
            //         Err(err) =>
            //     }
            // }

            let columns: Vec<String> = cap[2].split(',').map(|col| String::from(col)).collect(); //map all to trim func
                                                                                                 // let values : Vec<Type> = cap[3].split(',')
                                                                                                 //         .map(|val| {}).collect();

            // let res : Result<InsertQuery, ()>;
            if columns.len() != values.len() {
                res = Err(String::from("failed"));
            } else {
                res = Ok(Box::new(InsertQuery {
                    table_name,
                    columns,
                    values,
                }));
            }
        }
        res
    }
}
