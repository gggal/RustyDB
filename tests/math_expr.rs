extern crate pest;
#[macro_use]
extern crate pest_derive;

// use constants::GRAMMAR_FILE;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SQLParser;

#[test]
fn correctly_parse_math_expression() {
    let res = (
        "EQUATION",
        ((
            "MATH_EXPR",
            (
                "MATH_EXPR",
                ("OPERAND", ("NUMERIC_PRIMITIVE", 3)),
                ("ARITHM_OPERATOR", "*"),
                (
                    "MATH_PARANTHESES_EXPR",
                    (
                        "(",
                        (
                            "MATH_EXPR",
                            ("OPERAND", ("NUMERIC_PRIMITIVE", 1)),
                            ("ARITHM_OPERATOR", "+"),
                            (
                                "MATH_EXPR",
                                (
                                    "MATH_PARANTHESES_EXPR",
                                    "(",
                                    (
                                        "MATH_EXPR",
                                        ("OPERAND", ("NUMERIC_PRIMITIVE", 9)),
                                        ("ARITHM_OPERATOR", "-"),
                                        ("MATH_EXPR", ("OPERAND", ("NUMERIC_PRIMITIVE", 5))),
                                    ),
                                    ")",
                                ),
                                ("ARITHM_OPERATOR", "/"),
                                ("MATH_EXPR", ("OPERAND", ("NUMERIC_PRIMITIVE", 2))),
                            ),
                        ),
                        ")",
                    ),
                ),
            ),
            ("ARITHM_OPERATOR", "%"),
            ("MATH_EXPR", ("OPERAND", ("NUMERIC_PRIMITIVE", 2))),
        ),),
        ("CONDITIONAL_OPERATOR", "="),
        (
            "MATH_EXPR",
            ("OPERAND", ("NUMERIC_PRIMITIVE", -1)),
            ("ARITHM_OPERATOR", "+"),
            ("MATH_EXPR", ("OPERAND", ("NUMERIC_PRIMITIVE", 1))),
        ),
    );
    // assert!(SQLParser::parse(Rule::EQUATION, "3 * (1+(9-3)/2) % 2 = -1 + 1").is_ok())
    match SQLParser::parse(Rule::EQUATION, "3 * (1+(9-3)/2) % 2 = -1 + 1") {
        Err(_) => panic!(""),
        Ok(mut pairs) => {
            for pair in pairs {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
                println!("Text:    {}", pair.as_str());
                // A pair can be converted to an iterator of the tokens which make it up:
                for inner_pair in pair.into_inner() {
                    println!("My pair:  {}", inner_pair.into_inner());
                    match inner_pair.as_rule() {
                        _ => println!("Letter:  {}", inner_pair.as_str()),
                    };
                }
            }

            // println!("Here it is: {}", pairs.as_str());
            panic!("")
        }
    }

    fn compare_with_rule(parsed: Result(Pairs), expected: )
}
