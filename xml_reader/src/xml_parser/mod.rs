use super::types;

pub mod quick_xml;
pub mod minidom;
pub mod xml_rs;

pub trait XmlParser {
    fn parse(data: &String) -> types::ResourceMap;
}