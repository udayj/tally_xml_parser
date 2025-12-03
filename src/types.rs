use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Failed to parse XML: {0}")]
    XmlParse(#[from] roxmltree::Error),
}

pub type Result<T> = std::result::Result<T, ParserError>;

pub trait Parser {
    type Output;
    fn request_xml() -> String;
    fn parse(xml: &str) -> Result<Self::Output>;
}