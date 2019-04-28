use super::types;

pub mod quick_xml;
pub mod xml_rs;

pub trait XmlParser {
    fn parse(data: &String) -> types::ResourceMap;
}