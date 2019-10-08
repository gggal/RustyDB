use regex::Regex;

lazy_static! {
    pub static ref CREATE_QUERY_REGEX: Regex = Regex::new(
       r"(?ix:                                          #case-insensitive and ignore whitespace flags
    CREATE[[:space:]]+TABLE[[:space:]]+                 #CREATE TABLE
    (?P<table>[0-z_]+)[[:space:]]+                 #table name 
    \((?P<cols>([[:space:]]*.*[[:space:]]+(varchar|int|float)[[:space:]]*,)*            #columns
    ([[:space:]]*.*[[:space:]]+(varchar|int|float)[[:space:]]*))\)[[:space:]]*);$"
    )
    .expect("Internal error: cannot compile create query regex");

    pub static ref DELETE_QUERY_REGEX: Regex = Regex::new(
        r"(?ix:                                          #case-insensitive and ignore whitespace flags
        DELETE[[:space:]]+FROM[[:space:]]+              #DELETE FROM 
        (?P<table>[0-z_]+)[[:space:]]+                  #table name 
        WHERE[[:space:]]+(?P<tree>.*)[[:space:]]*);$" )
    .expect("Internal error: cannot compile delete query regex");

    pub static ref DROP_QUERY_REGEX: Regex = Regex::new(
        r"(?ix:                                           #case-insensitive and ignore whitespace flags
        DROP[[:space:]]+TABLE[[:space:]]+               #DROP TABLE
        (?P<table>[0-z_]+)[[:space:]]*);$"
        )
    .expect("Internal error: cannot compile drop query regex");
}
