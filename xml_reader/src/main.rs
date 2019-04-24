use std::fs;
use std::time::Instant;

mod xml_reader;
mod xml_reader_quick_xml;
mod xml_rs_reader;
mod types;

use xml_reader::XmlReader;
use xml_reader_quick_xml::XmlReaderQuickXml;


fn main() {
    profile_xml_reader::<XmlReaderQuickXml>("quick_xml");
    //profile_xml_rs();
}

fn profile_xml_reader<Reader: XmlReader>(reader_name: &str) {
    let data = get_xml_data();
    let now = Instant::now();
    let resource_map = Reader::read(data.as_bytes());
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("{} => bundles count: {}, seconds: {}", reader_name, resource_map.get_bundles_count(), sec);
}

fn get_xml_data() -> String {
    return fs::read_to_string("resource_map.xml").unwrap();
}