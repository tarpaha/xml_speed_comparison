extern crate xml;

use std::io::BufRead;
use xml::reader::{EventReader, XmlEvent};

use super::xml_reader::XmlReader;
use super::types;

pub struct XmlReaderXmlRs {
}

impl XmlReader for XmlReaderXmlRs {
    fn read<R: BufRead>(data: R) -> types::ResourceMap {
        return read_resource_map(data).unwrap();
    }
}

fn read_resource_map<R: BufRead>(data: R) -> Result<types::ResourceMap, String> {
    let mut resource_map = types::ResourceMap::new();
    let mut current_bundle: Option<types::Bundle> = None;
    let parser = EventReader::new(data);
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "Bundle" => {
                        let name = attributes.iter().find(|x| x.name.local_name.as_str() == "Filename");
                        if name == None {
                            return Err("Cannot find attribute Filename in Bundle tag".to_string());
                        }
                        let size = attributes.iter().find(|x| x.name.local_name.as_str() == "DownloadSize");
                        if size == None {
                            return Err("Cannot find attribute DownloadSize in Bundle tag".to_string());
                        }
                        match size.unwrap().value.parse::<u32>() {
                            Ok(size) => current_bundle = Some(types::Bundle::new(name.unwrap().value.to_string(), size)),
                            Err(e) => return Err(e.to_string())
                        }
                    },
                    "Asset" => {
                        let path_attr = attributes.iter().find(|x| x.name.local_name.as_str() == "AssetPath");
                        match path_attr {
                            Some(path) => {
                                match current_bundle {
                                    Some(ref mut bundle) => bundle.add_asset(types::Asset::new(path.value.to_string())),
                                    None => return Err("Found opening Asset tag out of Bundle scope".to_string())
                                }
                            }
                            None => return Err("Cannot find attribute AssetPath in Asset tag".to_string())
                        }
                    },
                    _ => {}
                }
            },
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "Bundle" => {
                        match current_bundle {
                            Some(_) => {
                                resource_map.add_bundle(current_bundle.unwrap());
                                current_bundle = None;
                            }
                            None => return Err("Found closing Bundle tag without opening one".to_string())
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    }
    Ok(resource_map)
}