### SQL queries parser

**Crate: https://crates.io/crates/my_sql_parser**

This parser is used to simplify `SELECT` queries and extract columns, table name and optional condition.

### Parsing process
The grammar defines how each part of a query is recognized:

- `SELECT` keyword introduces the list of columns.
- `FROM` marks the beginning of the table name.
- Optional `WHERE` clause can contain one or more conditions combined with `AND` or `OR`.
- Conditions can use `>`, `<`, `>=`, `<=`, `=`, `!=` operators and values can be names, numbers, or strings.
- `ORDER BY` clause can optionally sort by a column ascending (`ASC`) or descending (`DESC`).
- `INSERT INTO` allows adding new rows specifying columns and values.

### Grammar
```pest
WHITESPACE = _{ " " | "\t" | "\n" }

name = @{ (ASCII_ALPHANUMERIC | "_")+ }
number = @{ ASCII_DIGIT+ }
string = @{ "'" ~ (!"'" ~ ANY)* ~ "'" } 

comparison_operator = { ">=" | "<=" | "!=" | "=" | "<" | ">" }

condition = { name ~ comparison_operator ~ (name | number | string) }

multiple_conditions = { condition ~ (("AND" | "OR") ~ condition)* }

order_by = { "ORDER" ~ "BY" ~ name ~ ("ASC" | "DESC")? }

insert = { 
    "INSERT" ~ "INTO" ~ name ~ 
    "(" ~ name ~ ("," ~ name)* ~ ")" ~ 
    "VALUES" ~ 
    "(" ~ (name | number | string) ~ ("," ~ (name | number | string))* ~ ")" ~ ";"
}

query = { 
    "SELECT" ~ name ~ "FROM" ~ name ~ ( "WHERE" ~ multiple_conditions )? ~ ( order_by )? ~ ";"
}
```

### Sample Code

```rust
// Example input query
let query_str = "SELECT name FROM users WHERE age >= 18 ORDER BY age DESC;";

// Parse the query using the SQL queries parser
let parsed = sql_parser::Grammar::parse(sql_parser::Rule::query, query_str)
    .or_else(|_| sql_parser::Grammar::parse(sql_parser::Rule::insert, query_str))
    .or_else(|_| sql_parser::Grammar::parse(sql_parser::Rule::condition, query_str))
    .or_else(|_| sql_parser::Grammar::parse(sql_parser::Rule::multiple_conditions, query_str))
    .or_else(|_| sql_parser::Grammar::parse(sql_parser::Rule::order_by, query_str));

match parsed {
    Ok(pairs) => println!("Parsed: {:?}", pairs),
    Err(_) => eprintln!("Error: cannot parse query: {}", query_str),
}

// If we parse it, we would get something like this:
let result = Query {
    columns: vec!["name"],
    table: "users".to_string(),
    conditions: vec![
        Condition {
            field: "age".to_string(),
            operator: ">=".to_string(),
            value: "18".to_string(),
        }
    ],
    order_by: Some(OrderBy {
        field: "age".to_string(),
        direction: OrderDirection::Desc,
    }),
};
```