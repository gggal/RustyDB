RustyDB should consist of the following components:
1.parser -> parses some subset of sql
2.planner -> builds a plan for every query
3.storage engine -> stores the tables in the form of lists and indexes
4.???

Rusty's parser uses the Pest parser (https://github.com/pest-parser/pest) to parse 
SQL input and initializes SQL query objects

