use super::types;
use std::io::BufRead;

pub mod quick_xml;
pub mod xml_rs;

pub trait XmlParser {
    fn parse<R: BufRead>(data: R) -> types::ResourceMap;
}