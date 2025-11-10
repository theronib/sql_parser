use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[error("Parsing failed")]
    ParsingFailed,
}
