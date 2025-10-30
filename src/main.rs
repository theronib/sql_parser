use pest::Parser;
use sql_parser::*;

fn main() -> anyhow::Result<()> {
    let a = Grammar::parse(Rule::query, "SELECT name FROM users;")?;
    println!("{:?}", a);

    Ok(())
}
