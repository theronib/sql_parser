use anyhow::anyhow;
use pest::Parser;
use sql_parser::*;

#[test]
fn test_select() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::query, "SELECT name FROM users;")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "SELECT name FROM users;");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 23);

    Ok(())
}

#[test]
fn test_query_incorrect_missing_ident() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::query, "SELECT;");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_query_incorrect_empty() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::query, "");
    assert!(pair.is_err());
    Ok(())
}
