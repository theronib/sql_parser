use anyhow::anyhow;
use pest::Parser;
use my_sql_parser::*;

#[test]
fn test_mini_select() -> anyhow::Result<()> {
    let sql = "SELECT name FROM users;";
    let pair = Grammar::parse(Rule::query, sql)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), sql);
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), sql.len());
    Ok(())
}

#[test]
fn test_full_select() -> anyhow::Result<()> {
    let sql = "SELECT name FROM users WHERE age >= 18 AND name != 'John' ORDER BY age DESC;";
    let pair = Grammar::parse(Rule::query, sql)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), sql);
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), sql.len());
    Ok(())
}

#[test]
fn test_full_insert() -> anyhow::Result<()> {
    let sql = "INSERT INTO users(name, age) VALUES('Bob', 18);";
    let pair = Grammar::parse(Rule::insert, sql)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), sql);
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), sql.len());
    Ok(())
}
