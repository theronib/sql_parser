use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub mod error;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParserError;
    use pest::Parser;

    #[test]
    fn test_ident() -> Result<(), ParserError> {
        let mut pairs =
            Grammar::parse(Rule::name, "username").map_err(|_| ParserError::ParsingFailed)?;

        let pair = pairs
            .next()
            .ok_or(ParserError::UnexpectedToken("username".into()))?;
        assert_eq!(pair.as_str(), "username");
        Ok(())
    }

    #[test]
    fn test_number() -> Result<(), ParserError> {
        let mut pairs =
            Grammar::parse(Rule::number, "42").map_err(|_| ParserError::ParsingFailed)?;
        let pair = pairs
            .next()
            .ok_or(ParserError::UnexpectedToken("42".into()))?;
        assert_eq!(pair.as_str(), "42");
        Ok(())
    }

    #[test]
    fn test_string() -> Result<(), ParserError> {
        let mut pairs =
            Grammar::parse(Rule::string, "'Hello'").map_err(|_| ParserError::ParsingFailed)?;
        let pair = pairs
            .next()
            .ok_or(ParserError::UnexpectedToken("'Hello'".into()))?;
        assert_eq!(pair.as_str(), "'Hello'");
        Ok(())
    }

    #[test]
    fn test_comparison_operator() -> Result<(), ParserError> {
        let operators = [">=", "<=", "!=", "=", "<", ">"];
        for op in operators {
            let mut pairs = Grammar::parse(Rule::comparison_operator, op)
                .map_err(|_| ParserError::ParsingFailed)?;
            let pair = pairs
                .next()
                .ok_or(ParserError::UnexpectedToken(op.into()))?;
            assert_eq!(pair.as_str(), op);
        }
        Ok(())
    }

    #[test]
    fn test_condition() -> Result<(), ParserError> {
        let mut pairs =
            Grammar::parse(Rule::condition, "age >= 18").map_err(|_| ParserError::ParsingFailed)?;
        let pair = pairs
            .next()
            .ok_or(ParserError::UnexpectedToken("age >= 18".into()))?;
        assert_eq!(pair.as_str(), "age >= 18");

        let mut pairs2 = Grammar::parse(Rule::condition, "name != 'John'")
            .map_err(|_| ParserError::ParsingFailed)?;
        let pair2 = pairs2
            .next()
            .ok_or(ParserError::UnexpectedToken("name != 'John'".into()))?;
        assert_eq!(pair2.as_str(), "name != 'John'");
        Ok(())
    }

    #[test]
    fn test_multiple_conditions() -> Result<(), ParserError> {
        let mut pairs = Grammar::parse(Rule::multiple_conditions, "age > 18 AND name != 'John'")
            .map_err(|_| ParserError::ParsingFailed)?;
        let pair = pairs.next().ok_or(ParserError::UnexpectedToken(
            "age > 18 AND name != 'John'".into(),
        ))?;
        assert_eq!(pair.as_str(), "age > 18 AND name != 'John'");
        Ok(())
    }

    #[test]
    fn test_order_by() -> Result<(), ParserError> {
        let mut pairs = Grammar::parse(Rule::order_by, "ORDER BY age DESC")
            .map_err(|_| ParserError::ParsingFailed)?;
        let pair = pairs
            .next()
            .ok_or(ParserError::UnexpectedToken("ORDER BY age DESC".into()))?;
        assert_eq!(pair.as_str(), "ORDER BY age DESC");

        let mut pairs2 = Grammar::parse(Rule::order_by, "ORDER BY name ASC")
            .map_err(|_| ParserError::ParsingFailed)?;
        let pair2 = pairs2
            .next()
            .ok_or(ParserError::UnexpectedToken("ORDER BY name ASC".into()))?;
        assert_eq!(pair2.as_str(), "ORDER BY name ASC");
        Ok(())
    }

    #[test]
    fn test_insert() -> Result<(), ParserError> {
        let sql = "INSERT INTO users(name, age) VALUES('Bob', 18);";
        let mut pairs =
            Grammar::parse(Rule::insert, sql).map_err(|_| ParserError::ParsingFailed)?;
        let pair = pairs
            .next()
            .ok_or(ParserError::UnexpectedToken(sql.into()))?;
        assert_eq!(pair.as_str(), sql);
        Ok(())
    }

    #[test]
    fn test_name_incorrect_empty() {
        let result = Grammar::parse(Rule::name, "")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| pairs.next().ok_or(ParserError::UnexpectedToken("".into())));
        assert!(result.is_err());
    }

    #[test]
    fn test_number_incorrect_letters() {
        let result = Grammar::parse(Rule::number, "abc")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| {
                pairs
                    .next()
                    .ok_or(ParserError::UnexpectedToken("abc".into()))
            });
        assert!(result.is_err());
    }

    #[test]
    fn test_string_incorrect_unclosed() {
        let result = Grammar::parse(Rule::string, "'Hello")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| {
                pairs
                    .next()
                    .ok_or(ParserError::UnexpectedToken("'Hello".into()))
            });
        assert!(result.is_err());
    }

    #[test]
    fn test_condition_incorrect_missing_value() {
        let result = Grammar::parse(Rule::condition, "age >")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| {
                pairs
                    .next()
                    .ok_or(ParserError::UnexpectedToken("age >".into()))
            });
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_conditions_incorrect_missing_operator() {
        let result = Grammar::parse(Rule::multiple_conditions, "AND age > 18")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| {
                pairs
                    .next()
                    .ok_or(ParserError::UnexpectedToken("AND age > 18".into()))
            });
        assert!(result.is_err());
    }

    #[test]
    fn test_order_by_incorrect_missing_column() {
        let result = Grammar::parse(Rule::order_by, "ORDER BY")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| {
                pairs
                    .next()
                    .ok_or(ParserError::UnexpectedToken("ORDER BY".into()))
            });
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_incorrect_missing_values() {
        let sql = "INSERT INTO users(name, age);";
        let result = Grammar::parse(Rule::insert, sql)
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| pairs.next().ok_or(ParserError::UnexpectedToken(sql.into())));
        assert!(result.is_err());
    }

    #[test]
    fn test_query_incorrect_empty() {
        let result = Grammar::parse(Rule::query, "")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| pairs.next().ok_or(ParserError::UnexpectedToken("".into())));
        assert!(result.is_err());
    }

    #[test]
    fn test_query_incorrect_missing_ident() {
        let result = Grammar::parse(Rule::query, "SELECT;")
            .map_err(|_| ParserError::ParsingFailed)
            .and_then(|mut pairs| {
                pairs
                    .next()
                    .ok_or(ParserError::UnexpectedToken("SELECT;".into()))
            });
        assert!(result.is_err());
    }
}
