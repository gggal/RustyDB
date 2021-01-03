// /*
// create asd; //missing 'table' in create statement
// create table 'a_table'; // missing fields specification in create statement
// create table 'a_table' (col1 invalid_type); // invalid type in create statement
// create table 'a_table' (col1 int; //missing bracket in create statement
// create table 'a_table' (col1 int; col2 int); //invalid field separator
// create table 'a_table' (col1 int; col2 int) some_redundant_symbols; //invalid query structure; query expected "create table <table_name> (<arg_name> <arg_type>[, <arg_name> arg_type])"

// select * from 'a_table' // missing semicolon in statement
// select 123 from 'a_table'; //invalid table identifier
// unknown; //unknown command -> supported statements: select, insert ...
// select * from a_table; //ok; quotes can be ommitted
// select * from "a_table"; //ok; doublequotes are ok
// select * from "a_table; //missing quotation mark
// select  *   from a_table; //ok; extraspaces
// insert into a_table (arg2) ("Хей!") //cyrillic symbols in varchar
// division by zero

// insert 'a_table' (arg1) values (1); // missing 'into'
// insert into 'a_table' (arg1) values (1,2); // unconsistent number of values and args
// insert into 'a_table' (arg1, arg2) values (1); // unconsistent number of values and args
// insert into 'a_table' () values (); //ok
// insert into 'a_table' arg1 values 1; // invalid query structure;

// select t.arg1 from 'a_table' as a; //unknown alias t
// select a.arg1 from 'a_table' a; //ok; 'as' can be ommited
// select arg1 from 'a_table'; //ok; alias can be ommited if it's unambiguous
// select a.invalid_arg from 'a_table' as a; //invalid field for table
// select arg1 as 123 from 'a_table' as a; //invalid identifier

// select arg1 from 'a_table' as a where invalid_arg > 2; //invalid field for table 2
// select arg1 from 'a_table' as a where arg1 ? 2; //invalid constrain syntax -> supported operations...
// select arg1 from 'a_table' as a where arg1 + 2; //invalid constrain syntax2; constrains should be boolean
// select arg1 from 'a_table' as a where arg1 + 'a'; //invalid constrain syntax 3; invalid operand for operation '+'
// select arg1 from 'a_table' as a where; // empty where clause

// select a.arg1 from 'a_table' a join 'b_table' b; //missing on clause
// select a.arg1 from 'a_table' a join 'b_table' b on arg1; //ommitting table alias in on clause not supported
// select a.arg1 from 'a_table' a join 'b_table' a; //duplicate alias names for tables
// select a.arg1 from 'a_table' a join 'b_table'; //missing table alias

// update 'a_table' set arg1=1;// ommiting where clause results in updating all records in a table
// update 'a_table' arg1=1; //missing 'set' clause - invalid query structure
// update 'a_table' set arg1=1 and arg2=2; //invalid separator in set clause

// delete from 'a_table'; //ok - where clause can be ommitted
// delete 'a_table' //invalid query structure

// drop 'a_table'; //invalid query structure 'table' cannot be ommitted

// //select arg1 from 'a_table' as a where args1 > 2 and args2 < 1 || args2 > 20; // persist operator presedence

// */
// // ___create_statement___

// #[test]
// fn missing_table_clause() {
//     assert_eq!(CreateQuery::new("create table_name;"), "")
// }

// #[test]
// fn missing_columns_in_create() {
//     assert_eq!(CreateQuery::new("create table table_name;"), "")
// }

// #[test]
// fn invalid_type_in_create() {
//     assert_eq!(CreateQuery::new("create table_name (col1 unknown_type);"), "")
// }

// #[test]
// fn missing_bracket_in_create() {
//     assert_eq!(CreateQuery::new("create table_name (col1 varchar;"), "")
// }

// #[test]
// fn invalid_separator_in_create() {
//     assert_eq!(CreateQuery::new("create table_name (col1 varchar; col2 varchar);"), "")
// }

// #[test]
// fn redundant_symbols_in_create() {
//     assert_eq!(CreateQuery::new("create table_name (col1 varchar) asdasdsasad;"), "")
// }

// // ___select_statement___

// // select t.arg1 from 'a_table' as a; //unknown alias t - todo
// // select a.arg1 from 'a_table' a; //ok; 'as' can be ommited
// // select arg1 from 'a_table'; //ok; alias can be ommited if it's unambiguous -todo
// // select a.invalid_arg from 'a_table' as a; //invalid field for table -todo
// // select arg1 as 123 from 'a_table' as a; //invalid identifier

// // select arg1 from 'a_table' as a where invalid_arg > 2; //invalid field for table 2 -todo
// // select arg1 from 'a_table' as a where arg1 ? 2; //invalid constrain syntax -> supported operations...
// // select arg1 from 'a_table' as a where arg1 + 2; //invalid constrain syntax2; constrains should be boolean
// // select arg1 from 'a_table' as a where arg1 + 'a'; //invalid constrain syntax 3; invalid operand for operation '+'  -todo
// // select arg1 from 'a_table' as a where; // empty where clause

// // select a.arg1 from 'a_table' a join 'b_table' b; //missing on clause
// // select a.arg1 from 'a_table' a join 'b_table' b on arg1; //ommitting table alias in on clause not supported
// // select a.arg1 from 'a_table' a join 'b_table' a; //duplicate alias names for tables todo
// // select a.arg1 from 'a_table' a join 'b_table'; //missing table alias todo

// #[test]
// fn omitted_alias_select() {
//     assert_eq!(SelectQuery::new("select a.arg1 from 'a_table' a;"), SelectQuery{})
// }

// #[test]
// fn unknown_operator_in_where_clause() {
//     assert_eq!(SelectQuery::new("select arg1 from 'a_table' where arg1 ? 2;"), Err(""))
// }

// #[test]
// fn corrupt_filter_tree_in_where_clause_1() {
//     assert_eq!(SelectQuery::new("select arg1 from 'a_table' where arg1 + 2;"), "")
// }

// #[test]
// fn corrupt_filter_tree_in_where_clause_2() {
//     assert_eq!(SelectQuery::new("select arg1 from 'a_table' where 1 < 2 and arg1 + 2;"), "")
// }

// #[test]
// fn empty_where_clause() {
//     assert_eq!(SelectQuery::new("select 123 from 'a_table' where;"), "")
// }

// #[test]
// fn missing_where_clause() {
//     assert_eq!(SelectQuery::new("select 123 from 'a_table';"), Ok(_))
// }

// #[test]
// fn missing_on_in_join_clause() {
//     assert_eq!(SelectQuery::new("select a.arg1 from 'a_table' a join 'b_table' b;"), "")
// }

// #[test]
// fn ommitting_alias_in_join_clause() {
//     assert_eq!(SelectQuery::new("select a.arg1 from 'a_table' a join 'b_table' b on arg1;"), "")
// }

// // ___insert_statement___

// // insert 'a_table' (arg1) values (1); // missing 'into'
// // insert into 'a_table' (arg1) values (1,2); // inconsistent number of values and args
// // insert into 'a_table' (arg1, arg2) values (1); // inconsistent number of values and args
// // insert into 'a_table' () values (); //ok
// // insert into 'a_table' arg1 values 1; // invalid query structure;

// #[test]
// fn empty_cols_and_values() {
//     assert_eq!(InsertQuery::new("insert into 'a_table' () values ();"), Ok(_))
// }

// #[test]
// fn missing_into_word() {
//     assert_eq!(InsertQuery::new("insert 'a_table' (arg1) values (1);"), "")
// }

// #[test]
// fn inconsistent_number_of_values_and_args_1() {
//     assert_eq!(InsertQuery::new("insert into 'a_table' (arg1) values (1,2);"), "")
// }

// #[test]
// fn inconsistent_number_of_values_and_args_2() {
//     assert_eq!(InsertQuery::new("insert into 'a_table' (arg1, arg2) values (1);"), "")
// }

// #[test]
// fn missing_paranthesis() {
//     assert_eq!(InsertQuery::new("insert into 'a_table' arg1 values 1;"), "")
// }

// // ___update_statement___

// // update 'a_table' set arg1=1;// ommiting where clause results in updating all records in a table -todo
// // update 'a_table' arg1=1; //missing 'set' clause - invalid query structure
// // update 'a_table' set arg1=1 and arg2=2; //invalid separator in set clause

// #[test]
// fn missing_set_clause() {
//     assert_eq!(UpdateQuery::new("update 'a_table' arg1=1;"), "")
// }

// #[test]
// fn invalid_separator_in_set_clause() {
//     assert_eq!(UpdateQuery::new("update 'a_table' set arg1=1 and arg2=2;"), "")
// }

// // ___delete_statement___

// // delete from 'a_table'; //ok - where clause can be ommitted
// // delete 'a_table' //invalid query structure

// #[test]
// fn ommitted_where_clause() {
//     assert_eq!(DeleteQuery::new("delete from 'a_table';"), Ok(_));
// }

// #[test]
// fn missing_from_word_in_delete_statement() {
//     assert_eq!(DeleteQuery::new("delete 'a_table';"), Err());
// }

// // ___drop_statement___

// // drop 'a_table'; //invalid query structure 'table' cannot be ommitted

// #[test]
// fn missing_from_word_in_drop_statement() {
//     assert_eq!(DropQuery::new("drop 'a_table';"), Err());
// }
