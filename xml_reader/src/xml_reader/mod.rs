use super::types;
use std::io::BufRead;

pub mod quick_xml;
pub mod xml_rs;

pub trait XmlReader {
    fn read<R: BufRead>(data: R) -> types::ResourceMap;
}