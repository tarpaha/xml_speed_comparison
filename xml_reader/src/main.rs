use std::fs;
use std::time::Instant;

mod xml_parser;
mod types;

use xml_parser::XmlParser;
use xml_parser::quick_xml::XmlParserQuickXml;
use xml_parser::xml_rs::XmlParserXmlRs;

fn main() {
    profile_xml_parser::<XmlParserQuickXml>("quick_xml");
    profile_xml_parser::<XmlParserXmlRs>   ("xml_rs   ");
}

fn profile_xml_parser<Parser: XmlParser>(parser_name: &str) {
    let data = get_xml_data();
    let now = Instant::now();
    let resource_map = Parser::parse(&data);
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("{} => bundles count: {}, seconds: {}", parser_name, resource_map.get_bundles_count(), sec);
}

fn get_xml_data() -> String {
    return fs::read_to_string("resource_map.xml").unwrap();
}