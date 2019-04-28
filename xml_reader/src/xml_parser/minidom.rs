use minidom::Element;

use super::XmlParser;
use super::types;

pub struct XmlParserMinidom;

impl XmlParser for XmlParserMinidom {
    fn parse(data: &String) -> types::ResourceMap {
        let mut resource_map = types::ResourceMap::new();
        let root: Element = data.as_str().parse().unwrap();
        let bundles_xml = root.children().next().unwrap();
        for bundle_xml in bundles_xml.children() {
            let filename = bundle_xml.attr("Filename").unwrap().to_string();
            let download_size = bundle_xml.attr("DownloadSize").unwrap().parse().unwrap();
            let mut bundle = types::Bundle::new(filename, download_size);
            let assets_xml = bundle_xml.children().next().unwrap(); 
            for asset_xml in assets_xml.children() {
                let asset_path = asset_xml.attr("AssetPath").unwrap().to_string();
                let asset = types::Asset::new(asset_path);
                bundle.add_asset(asset);
            }
            resource_map.add_bundle(bundle);
        }
        return resource_map;
    }
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
              <Assets>
                <Asset AssetPath="asset01" />
              </Assets>
            </Bundle>
          </Bundles>
        </ResourceMapData>
        "#;
        let resource_map = XmlParserMinidom::parse(&xml.to_string());
        assert_eq!(resource_map.get_bundles_count(), 1);
        let bundle = resource_map.get_bundle(0);
        assert_eq!(bundle.get_filename(), "bundle01");
        assert_eq!(bundle.get_download_size(), 42);
        assert_eq!(bundle.get_assets_count(), 1);
        let asset = bundle.get_asset(0);
        assert_eq!(asset.get_path(), "asset01");
    }
}