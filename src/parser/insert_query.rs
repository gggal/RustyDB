use filter::Type;
use regex::Regex;

// #[derive(Debug)]
pub struct InsertQuery {
    columns: Vec<String>,
    values: Vec<Type>,
    table_name: String,
}

impl InsertQuery {
    pub fn new(text: &str) -> Result<Box<InsertQuery>, String> {
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

#[cfg(test)]
mod tests {
    use parser::InsertQuery;

    #[test]
    fn empty_cols_and_values() {
        assert!(InsertQuery::new("insert into 'a_table' () values ();").is_ok())
    }

    #[test]
    fn missing_into_word() {
        assert!(InsertQuery::new("insert 'a_table' (arg1) values (1);").is_err())
    }

    #[test]
    fn inconsistent_number_of_values_and_args_1() {
        assert!(InsertQuery::new("insert into 'a_table' (arg1) values (1,2);").is_err())
    }

    #[test]
    fn inconsistent_number_of_values_and_args_2() {
        assert!(InsertQuery::new("insert into 'a_table' (arg1, arg2) values (1);").is_err())
    }

    #[test]
    fn missing_paranthesis() {
        assert!(InsertQuery::new("insert into 'a_table' arg1 values 1;").is_err())
    }
}
