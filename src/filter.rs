//pub mod parser {
use std::collections::HashMap;

/*
so what do i need when parsing in terms of binding/variable structures:
for CreateQuery -> just type for verification verification e.g. is the word int a valid type
for InsertQuery -> it needs to be checked if the value given conforms to the type of every column
for Filter
*/

///A filter tree
pub struct FilterTree {
    root: FilterNode,
    vars: HashMap<String, Var>,
}

//todo var, date
#[derive(Debug)]
pub enum Var {
    INT(i64),
    DOUBLE(f64),
    VARCHAR(String),
    NULL,
}

#[derive(Debug)]
pub enum Type {
    INT,
    DOUBLE,
    VARCHAR,
    TS,
    NULL,
}

impl Type {
    pub fn parse(text: &str) -> Result<Type, &'static str> {
        match text {
            "i?:int" => Ok(Type::INT),
            "i?:double" => Ok(Type::DOUBLE),
            "i?:varchar" => Ok(Type::VARCHAR),
            "i?:timestamp" => Ok(Type::TS),
            "i?:null" => Ok(Type::NULL),
            _ => Err("Unsupported type was found"),
        }
    }
}

enum Operand {
    CONST(Var),
    VAR(String),
    NODE(Box<FilterNode>),
}

impl Var {
    fn str_to_type(text: &str) -> Result<Var, &str> {
        let chrs: Vec<char> = text.chars().collect();
        let first: char = *chrs.first().unwrap();
        let last: char = *chrs.last().unwrap();
        let s: String = String::from(text);
        if regex::Regex::new(r"i?=null").unwrap().is_match(text) {
            Ok(Var::NULL)
        } else {
            match (first, last) {
                ('\'', '\'') => Ok(Var::VARCHAR(chrs[1..chrs.len() - 2].into_iter().collect())),
                ('\"', '\"') => Ok(Var::VARCHAR(chrs[1..chrs.len() - 2].into_iter().collect())),
                _real_num if chrs.contains(&'.') => Ok(Var::DOUBLE(s.parse().unwrap())),
                _num => Ok(Var::INT(s.parse().unwrap())),
            }
        }
    }
}

impl<'a> FilterTree {
    /// Constructs a filter tree from the 'where' clause in a query
    pub fn new(text: &str) -> Result<FilterTree, String> {
        //iterate the tree and get all variables in it and put them in the map
        match FilterNode::new(text) {
            Some(root) => Ok(FilterTree {
                root,
                vars: HashMap::new(),
            }),
            None => Err("".to_string()),
        }
    }

    fn execute(&mut self) -> Operand {
        self.root.evaluate(&self.vars);
        self.root.execute()
    }
}

impl Operand {
    fn parse_operand(it: &mut std::str::Chars) -> Option<Operand> {
        let mut opened = 0_i32;
        let mut closed = 0_i32;
        let mut res: String = String::new();
        let mut to_return: Option<Operand> = None;
        if it.next().unwrap() == '(' {
            loop {
                if it.next().unwrap() == ')' {
                    closed += 1;
                }
                if it.next().unwrap() == '(' {
                    opened += 1;
                }

                if it.next().unwrap() == ')' && opened == closed {
                    break;
                }
                res.push(it.next().unwrap());
            }
            if let Some(node) = FilterNode::new(&res) {
                to_return = Some(Operand::NODE(Box::new(node)));
            }
        } else {
            loop {
                if it.next().unwrap() == ' ' {
                    break;
                }
                res.push(it.next().unwrap());
            }
            match Var::str_to_type(&res) {
                Err(_) => {
                    to_return = Some(Operand::VAR(res));
                }
                Ok(t) => {
                    to_return = Some(Operand::CONST(t));
                }
            }
        }
        to_return
    }
}

enum Operator {
    ADD(fn(i64, i64) -> i64),
    SUBSTRACT(fn(i32, i32) -> i32),
    MULTIPLY(fn(i32, i32) -> i32),
    DIVIDE(fn(i32, i32) -> i32),
    EQUALS(fn(i32, i32) -> bool),
    LT(fn(i32, i32) -> bool),
    GT(fn(i32, i32) -> bool),
    AND(fn(bool, bool) -> bool),
    OR(fn(bool, bool) -> bool),
    NOT(fn(bool) -> bool),
}

impl Operator {
    //TODO change name to new?
    fn parse_operator(text: &mut std::str::Chars) -> Option<Operator> {
        let t: String = text.collect();
        let first_word: &str = t.split_whitespace().nth(0).unwrap();
        match first_word {
            "+" => {
                text.nth(1);
                Some(Operator::ADD(|a: i64, b: i64| a + b))
            }
            "-" => {
                text.nth(1);
                Some(Operator::SUBSTRACT(|a: i32, b: i32| a - b))
            }
            "*" => {
                text.nth(1);
                Some(Operator::MULTIPLY(|a: i32, b: i32| a * b))
            }
            "/" => {
                text.nth(1);
                Some(Operator::DIVIDE(|a: i32, b: i32| a / b))
            }
            "=" => {
                text.nth(1);
                Some(Operator::EQUALS(|a: i32, b: i32| a == b))
            }
            "<" => {
                text.nth(1);
                Some(Operator::LT(|a: i32, b: i32| a < b))
            }
            ">" => {
                text.nth(1);
                Some(Operator::GT(|a: i32, b: i32| a > b))
            }
            "(i!=)AND" => {
                text.nth(1);
                Some(Operator::AND(|a: bool, b: bool| a && b))
            }
            "(i!=)OR" => {
                text.nth(1);
                Some(Operator::OR(|a: bool, b: bool| a || b))
            }
            "(i!=)NOT" => {
                text.nth(1);
                Some(Operator::NOT(|a: bool| !a))
            } //TODO remove !
            _ => None,
        }
    }

    fn get_precedence(&self) -> i32 {
        match self {
            Operator::ADD(_)
            | Operator::SUBSTRACT(_)
            | Operator::MULTIPLY(_)
            | Operator::DIVIDE(_) => 1,
            Operator::EQUALS(_) | Operator::LT(_) | Operator::GT(_) => 2,
            Operator::AND(_) | Operator::OR(_) | Operator::NOT(_) => 3,
        }
    }

    // TODO metaprogramming?
    fn execute(&self, op1: &Operand, op2: &Operand) -> Operand {
        match (self, op1, op2) {
            (
                Operator::ADD(add_f),
                Operand::CONST(Var::INT(val1)),
                Operand::CONST(Var::INT(val2)),
            ) => Operand::CONST(Var::INT(add_f(*val1, *val2))),
            _ => Operand::CONST(Var::INT(0_i64)), //TODO add other operations
        }
    }
}

struct FilterNode {
    operator: Operator,
    operand1: Operand,
    operand2: Operand,
}

impl FilterNode {
    fn new(text: &str) -> Option<FilterNode> {
        let mut it: std::str::Chars = text.chars();
        let mut op1: Operand = Operand::parse_operand(&mut it).unwrap();
        loop {
            match (
                Operator::parse_operator(&mut it),
                Operand::parse_operand(&mut it),
            ) {
                (Some(op), Some(str2)) => {
                    op1 = Operand::NODE(Box::new(FilterNode {
                        operand1: op1,
                        operator: op,
                        operand2: str2,
                    }));
                }
                _ => break,
            }
        }
        None
        //consume first operand, consume operator, consume sec operand, make a node, next
    }

    pub fn is_leaf(&self) -> bool {
        true
    }

    fn execute(&self) -> Operand {
        match &self.operand1 {
            Operand::NODE(boxed_node1) => match &self.operand2 {
                Operand::NODE(boxed_node1) => self
                    .operator
                    .execute(&(*boxed_node1).execute(), &(*boxed_node1).execute()),
                other => self.operator.execute(&(*boxed_node1).execute(), other),
            },
            other => match &self.operand2 {
                Operand::NODE(boxed_node2) => {
                    self.operator.execute(other, &(*boxed_node2).execute())
                }
                other_2 => self.operator.execute(other, &other_2),
            },
        }
    }

    /// Binds the variables with the fields from the current row,
    /// the tree is being traversed in DFS matter and evaluated
    fn evaluate(&mut self, bindings: &HashMap<String, Var>) {
        match &mut self.operand1 {
            Operand::VAR(var_name) => {
                // let str = &var_name;
                let binding = bindings[var_name].clone();
                self.operand1 = Operand::CONST(binding);
            }
            Operand::NODE(boxed_node) => {
                let nod: &mut FilterNode = &mut **boxed_node;
                nod.evaluate(bindings);
            }
            Operand::CONST(_) => {}
        };
        //at this point there shouldn't be any unbind variable in the tree - TODO
    }
}

impl Clone for Var {
    fn clone(&self) -> Self {
        match self {
            Var::INT(i) => Var::INT(*i),
            Var::DOUBLE(d) => Var::DOUBLE(*d),
            Var::VARCHAR(v) => Var::VARCHAR(String::from(v)),
            _ => Var::NULL,
        }
    }
}
