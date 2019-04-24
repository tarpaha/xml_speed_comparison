use std::fs;
use std::time::Instant;

mod quick_xml_reader;
mod types;

fn main() {
    let data = read_file_to_string("resource_map.xml");

    let now = Instant::now();
    let resource_map = quick_xml_reader::read(data.as_bytes());
    let elapsed = now.elapsed();
    
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Bundles count: {}, seconds: {}", resource_map.get_bundles_count(), sec);
}

fn read_file_to_string(filename: &str) -> String {
    return fs::read_to_string(filename).unwrap();
}