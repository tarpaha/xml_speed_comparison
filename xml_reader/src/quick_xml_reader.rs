use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use std::io::{BufRead, BufReader};
use std::borrow::Cow;
use std::fs::File;
use std::str;

use super::types;

pub fn read_from_file(filename: &str) -> types::ResourceMap {
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    return read(file);
}

fn read<R: BufRead>(data: R) -> types::ResourceMap {
    let mut resource_map = types::ResourceMap::new();
    let mut current_bundle: Option<types::Bundle> = None;
    let mut reader = Reader::from_reader(data);
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"Bundle" => {
                        let filename = get_string_attribute(e, b"Filename"); 
                        let download_size = get_u32_attribute(e, b"DownloadSize");
                        current_bundle = Option::Some(types::Bundle::new(filename, download_size));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"Asset" => {
                        match current_bundle {
                            Some(ref mut bundle) => {
                                let asset_path = get_string_attribute(e, b"AssetPath");
                                bundle.add_asset(types::Asset::new(asset_path));
                            },
                            None => panic!("Opening Asset tag outside of Bundle scope")
                        }
                    },
                    _ => ()
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"Bundle" => {
                        match current_bundle  {
                            Some(_) => {
                                resource_map.add_bundle(current_bundle.unwrap());
                                current_bundle = None;
                            },
                            None => panic!("Closing Bundle tag without opening one") 
                        }
                    },
                    _ => ()
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => ()
        }
        buf.clear();
    }
    resource_map
}

fn get_string_attribute(tag: &BytesStart, name: &[u8]) -> String {
    let bytes = tag.attributes()
        .map(|a| a.unwrap())
        .find(|a| a.key == name)
        .unwrap()
        .value;
    return cow_chars_to_string(&bytes);   
}

fn get_u32_attribute(tag: &BytesStart, name: &[u8]) -> u32 {
    let bytes = tag.attributes()
        .map(|a| a.unwrap())
        .find(|a| a.key == name)
        .unwrap()
        .value;
    return cow_chars_to_u32(&bytes);
}

fn cow_chars_to_u32(chars: &Cow<[u8]>) -> u32 {
    let mut result = 0;
    for ch in chars.iter() {
        result *= 10;
        result += (ch - 48) as u32;
    }
    result 
}

fn cow_chars_to_string(chars: &Cow<[u8]>) -> String {
    return str::from_utf8(chars).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_reading() {
        let xml = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <ResourceMapData>
          <Bundles>
            <Bundle Filename="bundle01" DownloadSize="42">
              <Asset AssetPath="asset01" />
            </Bundle>
          </Bundles>
        </ResourceMapData>
        "#;
        let resource_map = read(xml.as_bytes());
        assert_eq!(resource_map.get_bundles_count(), 1);
        let bundle = resource_map.get_bundle(0);
        assert_eq!(bundle.get_filename(), "bundle01");
        assert_eq!(bundle.get_download_size(), 42);
        assert_eq!(bundle.get_assets_count(), 1);
        let asset = bundle.get_asset(0);
        assert_eq!(asset.get_path(), "asset01");
    }
}