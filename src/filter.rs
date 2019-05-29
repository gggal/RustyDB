//pub mod parser {
use std::collections::HashMap;


///A filter tree 
pub struct filter_tree {
    root : filter_node,
    //vars : HashMap<String, &'a mut Option<Type>>
}

enum Type {
    INT(i64),
    DOUBLE(f64),
    VARCHAR(String),
    NULL
}

enum Operand {
    CONST(Type),
    VAR(String),
    NODE(Box<filter_node>)
}

fn str_to_type(text : &str) -> Result<Type, &str> {
    let chrs : Vec<char> = text.chars().collect(); 
    let first : char = *chrs.first().unwrap();
    let last : char = *chrs.last().unwrap();
    let s : String = String::from(text);
    if regex::Regex::new(r"i?=null").unwrap().is_match(text) {
        Ok(Type::NULL)
    } else {
        match (first, last) {
            ('\'','\'') => Ok(Type::VARCHAR(chrs[1..chrs.len() - 2].into_iter().collect())),
            ('\"','\"') => Ok(Type::VARCHAR(chrs[1..chrs.len() - 2].into_iter().collect())),
            real_num if chrs.contains(&'.') => Ok(Type::DOUBLE(s.parse().unwrap())),
            num => Ok(Type::INT(s.parse().unwrap()))
        } 
    } 
}

impl<'a> filter_tree {
    /// Constructs a filter tree from the 'where' clause in a query
    pub fn new(text: &str) -> Result<filter_tree, String> {
        //iterate the tree and get all variables in it and put them in the map
        match filter_node::new(text) {
            Some(root) => Ok(filter_tree{root}),
            None => Err("".to_string())
        }
    }
    /// Binds the variables with the fields from the current row,
    /// the tree is being traversed in DFS matter and evaluated
    fn evaluate(&self, bindings : HashMap<String, Type>) -> bool {
        self.execute()
    }

    fn execute(&self) -> Option<Type> {
        if self.root.is_leaf() {
            self.root[operand].execute(operand1, operand2)
        } else {
            self.root[operand].execute(execute(operand1), execute(operand2))
        }
    }
}

fn parse_operand(it: &mut std::str::Chars) -> Option<Operand> {
    let mut opened = 0_i32;
    let mut closed = 0_i32;
    let mut res : String = String::new();
    let mut to_return : Option<Operand> = None;
    if it.next().unwrap()=='(' {
        while true {
            if it.next().unwrap()==')' {
                closed+=1;
            }
            if it.next().unwrap()=='(' {
                opened+=1;
            }

            if it.next().unwrap()==')' && opened==closed {
                break;
            }
            res.push(it.next().unwrap());
        }
        if let Some(node) = filter_node::new(&res) {
            to_return = Some(Operand::NODE(Box::new(node)));
        }
    } else {
        while true {
            if it.next().unwrap()==' ' {
                break;
            }
            res.push(it.next().unwrap());
        }
        match str_to_type(&res) {
            Err(_) => {to_return = Some(Operand::VAR(res));}
            Ok(t) => {to_return = Some(Operand::CONST(t));}
        }
    }
    to_return
} 

fn parse_operator(text: &mut std::str::Chars) -> Option<Operator> {
    let t : String = text.collect();
    let first_word : &str = t.split_whitespace().nth(0).unwrap();
    match first_word {
        "+" => {text.nth(1); Some(Operator::ADD)},
        "-" => {text.nth(1); Some(Operator::SUBSTRACT)},
        "*" => {text.nth(1); Some(Operator::MULTIPLY)},
        "/" => {text.nth(1); Some(Operator::DIVIDE)},
        "=" => {text.nth(1); Some(Operator::EQUALS)},
        "<" => {text.nth(1); Some(Operator::LT)},
        ">" => {text.nth(1); Some(Operator::GT)},
        "(i!=)AND" => {text.nth(1); Some(Operator::AND)},
        "(i!=)OR" => {text.nth(1); Some(Operator::OR)},
        "(i!=)NOT" => {text.nth(1); Some(Operator::NOT)},
        _ => None
    } 
} 

enum Operator {
    ADD,
    SUBSTRACT,
    MULTIPLY,
    DIVIDE,
    EQUALS,
    LT,
    GT,
    AND,
    OR,
    NOT
}

impl Operator {
    fn get_precedence(&self) -> i32 {
        match self {
            Operator::ADD|Operator::SUBSTRACT|Operator::MULTIPLY|Operator::DIVIDE => 1,
            Operator::EQUALS|Operator::LT|Operator::GT => 2,
            Operator::AND|Operator::OR|Operator::NOT => 3
        }
    }
}

struct filter_node {
    operator : Operator,
    operand1 : Operand,
    operand2 : Operand
}

impl filter_node {
    fn new(text: &str) -> Option<filter_node> {
            let mut it : std::str::Chars = text.chars();
            let mut op1 : Operand = parse_operand(&mut it).unwrap();
            while true {
                match (parse_operator(&mut it), parse_operand(&mut it)) {
                    (Some(op), Some(str2)) => {
                        op1 = Operand::NODE(Box::new(filter_node{operand1: op1, operator: op, operand2: str2}));
                    }
                    _ => break
                } 
            }
            None
            //consume first operand, consume operator, consume sec operand, make a node, next
        }
}
// }