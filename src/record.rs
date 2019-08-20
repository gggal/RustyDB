// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::io::Lines;

// enum Type {
//     INT(i64),
//     DOUBLE(f64),
//     VARCHAR(String),
//     NULL,
// }

// fn str_to_type(text: &str) -> Result<Type, &str> {
//     let chrs: Vec<char> = text.chars().collect();
//     let first: char = *chrs.first().unwrap();
//     let last: char = *chrs.last().unwrap();
//     let s: String = String::from(text);
//     if regex::Regex::new(r"i?=null").unwrap().is_match(text) {
//         Ok(Type::NULL)
//     } else {
//         match (first, last) {
//             ('\'', '\'') => Ok(Type::VARCHAR(chrs[1..chrs.len() - 2].into_iter().collect())),
//             ('\"', '\"') => Ok(Type::VARCHAR(chrs[1..chrs.len() - 2].into_iter().collect())),
//             real_num if chrs.contains(&'.') => Ok(Type::DOUBLE(s.parse().unwrap())),
//             num => Ok(Type::INT(s.parse().unwrap())),
//         }
//     }
// }

// struct Record {
//     pairs: HashMap<String, Type>,
//     str: Lines<String>,
//     iter: Option<std::io::Lines<'a, BufferedReader<R>>>,
// }

// impl Record {
//     fn new(text: String, names: Vec<(String, String)>) -> Option<Record> {
//         let pairs: HashMap<String, Type> = HashMap::new();

//         let values: Vec<&str> = Vec::new();
//         for val in text.split(",") {
//             values.push(val);
//         }

//         let mut invalid: bool = false;
//         for i in 1..names.len() {
//             match (names[i], str_to_type(values[i])) {
//                 ((String::from("VARCHAR"), name), Ok(Type::VARCHAR(a))) => {
//                     pairs.insert(names[i], Type::VARCHAR(a));
//                 }
//                 ((String::from("INT"), name), Ok(Type::INT(a))) => {
//                     pairs.insert(names[i], Type::INT(a));
//                 }
//                 ((String::from("FLOAT"), name), Ok(Type::DOUBLE(float))) => {
//                     pairs.insert(names[i], Type::DOUBLE(float));
//                 }
//                 _ => {
//                     invalid = true;
//                 }
//             }
//         }

//         Some(Record { pairs })
//     }

//     fn to_string(&self) -> String {
//         let mut to_return: String = String::new();
//         for (name, val) in &self.pairs {
//             to_return.push_str(&name);
//             to_return.push_str(" ");
//         }
//         to_return
//     }

//     fn retain(&self, to_retain: Vec<String>) {
//         self.pairs.retain(|pair| to_retain.contains(pair))
//     }

//     fn get(&self, name: String) -> Type {
//         self.get(name)
//     }
// }

// impl Iterator for Record {
//     fn next(&mut self) -> Option<u64> {
//         self.str.next();
//         self = Record::new(fs_manager.get)
//     }
// }
