use regex::Regex;

lazy_static! {
    pub static ref CREATE_QUERY_REGEX: Regex = Regex::new(
        r"
        (?ix)                                                          #case-insensitive and ignore whitespace flags
        create[[:space:]]+table[[:space:]]+                             #CREATE TABLE 
        (P<table>?[[:alnum:]]*)[[:space:]]+                             #TABLE NAME
        \((P<cols>([[:space:]]*.*[[:space:]]+[[:alnum:]]*[[:space:]]*,)*
        ([[:space:]]*.*[[:space:]]+[[:alnum:]]*[[:space:]]*))\)[[:space:]]*&"
    )
    .expect("Internal error: cannot compile create query regex");
}
