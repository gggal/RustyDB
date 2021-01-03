extern crate pest;
#[macro_use]
extern crate pest_derive;

// use constants::GRAMMAR_FILE;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SQLParser;

#[test]
fn correctly_parse_comments() {
    assert!(SQLParser::parse(Rule::COMMENT, "/**/").is_ok())
}

#[test]
fn fail_parsing_invalid_comments() {
    assert!(SQLParser::parse(Rule::COMMENT, "//").is_err())
}

#[test]
fn fail_parsing_lowercase_false() {
    assert!(SQLParser::parse(Rule::LOGICAL_PRIMITIVE, "false").is_err())
}

#[test]
fn fail_parsing_lowercase_true() {
    assert!(SQLParser::parse(Rule::LOGICAL_PRIMITIVE, "True").is_err())
}

#[test]
fn correctly_parse_false() {
    assert!(SQLParser::parse(Rule::LOGICAL_PRIMITIVE, "FALSE").is_ok())
}

#[test]
fn correctly_parse_true() {
    assert!(SQLParser::parse(Rule::LOGICAL_PRIMITIVE, "TRUE").is_ok())
}

#[test]
fn correctly_parse_positive_int() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "123").is_ok())
}

#[test]
fn correctly_parse_negative_int() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "-123").is_ok())
}

#[test]
fn correctly_parse_zero() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "0").is_ok())
}

#[test]
fn correctly_parse_real_number() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "-10.123").is_ok())
}

#[test]
fn correctly_parse_num_in_exponent_notation() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "6.022e23").is_ok())
}

#[test]
fn correctly_parse_num_in_exponent_notation_1() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "6.022E23").is_ok())
}

#[test]
fn fail_parsing_non_number() {
    assert!(SQLParser::parse(Rule::NUMERIC_PRIMITIVE, "a123").is_err())
}

#[test]
fn correctly_parse_leteral() {
    assert!(SQLParser::parse(Rule::LITERAL, "\"string\"").is_ok())
}

#[test]
fn correctly_parse_empty_leteral() {
    assert!(SQLParser::parse(Rule::LITERAL, "\"\"").is_ok())
}

#[test]
fn fail_parsing_non_literal() {
    assert!(SQLParser::parse(Rule::LITERAL, "string").is_err())
}

#[test]
fn correctly_parse_null() {
    assert!(SQLParser::parse(Rule::NULL_PRIMITIVE, "NULL").is_ok())
}

#[test]
fn fail_parsing_non_null() {
    assert!(SQLParser::parse(Rule::NULL_PRIMITIVE, "notnull").is_err())
}

#[test]
fn correctly_parse_numeric_primitive() {
    match SQLParser::parse(Rule::PRIMITIVE, "123") {
        Err(_) => panic!("Parsing num primitive failed"),
        Ok(mut pairs) => {
            match pairs.next() {
                None => panic!("Parsing num primitive failed"),
                Some(parsed) => {
                    assert_eq!(parsed.as_str(), "123");
                    // assert!(pairs.next().is_some());
                    assert_eq!(None, pairs.next());
                }
            }
        }
    }
}

#[test]
fn correctly_parse_identifier() {
    assert!(SQLParser::parse(Rule::IDENTIFIER, "_Asd123_").is_ok())
}

#[test]
fn fail_parsing_non_identifier() {
    assert!(SQLParser::parse(Rule::IDENTIFIER, "%not%valid").is_err())
}

// #[test]
// fn fail_parsing_invalid_comments_2() {
// let pairs = SQLParser::parse(Rule::COMMENT, "/**/asd").unwrap_or_else(|e| panic!("{}", e));
// for pair in pairs {
//     println!("Rule:    {:?}", pair.as_rule());
//     println!("Span:    {:?}", pair.as_span());
//     println!("Text:    {}", pair.as_str());
// }
// let tokens: Vec<_> = pairs.tokens().collect();

// assert_eq!(tokens.len(), 2);
// panic!("ASASAS")
// match SQLParser::parse(Rule::COMMENT, "/**/") {
//     Err(_) => panic!("Parsing comments failed"),
//     Ok(pairs) => {
//         println!("Execc test");
//         println!("Fist pair is {}", pairs);
//         panic!("ASASAS")
//         // assert!(pairs.next().is_some());
//         // assert_eq!(None, pairs.next());
//     }

// }
// }
