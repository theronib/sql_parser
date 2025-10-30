### SQL queries parser

This parser is used to simplify `SELECT` queries and extract columns, table name and optional condition.

### Parsing process
The grammar defines how each part of a query is recognized:
    SELECT keyword introduces the list of columns.
    FROM marks the beginning of the table name.


### Sample Code

```rust
// Example input query
let query_str = "SELECT name FROM users;";

// Parse the query using the SQL queries parser
let parsed = sql_parser::Grammar::parse(sql_parser::Rule::query, query_str)
    .expect("parsing failed")
    .next()
    .expect("no pair");

// If we parse it, we would get something like this:
let result = Query {
    columns: vec!["name"],       
    table: "users".to_string(),  
    conditions: vec![], 
};
```