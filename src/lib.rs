use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_ident() {
        let pair = Grammar::parse(Rule::ident, "username")
            .expect("parsing failed")
            .next()
            .expect("no pair");

        assert_eq!(pair.as_str(), "username");
    }
}
