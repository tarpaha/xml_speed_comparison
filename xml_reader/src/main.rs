mod quick_xml_reader;
mod types;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    //let resource_map = quick_xml_reader::read_from_test_string();
    let resource_map = quick_xml_reader::read_from_file("resource_map.xml");

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Bundles count: {}, seconds: {}", resource_map.get_bundles_count(), sec);
}