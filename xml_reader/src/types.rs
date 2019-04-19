#[derive(Debug)]
pub struct Asset {
    path: String
}

impl Asset {
    pub fn new(path: String) -> Asset {
        Asset { path }
    }
}

#[derive(Debug)]
pub struct Bundle {
    name: String,
    size: u32,
    assets: Vec<Asset>
}

impl Bundle {
    pub fn new(name: String, size: u32) -> Bundle {
        Bundle { name, size, assets: vec![] }
    }
    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }
}

#[derive(Debug)]
pub struct ResourceMap {
    bundles: Vec<Bundle>
}

impl ResourceMap {
    pub fn new() -> ResourceMap {
        ResourceMap{ bundles: vec![] }
    }
    pub fn add_bundle(&mut self, bundle: Bundle) {
        self.bundles.push(bundle);
    }
    pub fn get_bundles_count(&self) -> usize {
        return self.bundles.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_creation() {
        let path = "path";
        let asset = Asset::new(path.to_string());
        assert_eq!(asset.path, path);
    }

    #[test]
    fn test_bundle_creation() {
        let name = "name";
        let size = 42;
        let bundle = Bundle::new(name.to_string(), size);
        assert_eq!(bundle.name, name);
        assert_eq!(bundle.size, size);
        assert_eq!(bundle.assets.len(), 0);
    }

    #[test]
    fn test_resource_map_creation() {
        let resource_map = ResourceMap::new();
        assert_eq!(resource_map.bundles.len(), 0);
    }

    #[test]
    fn test_bundle_add_asset() {
        let path = "path";
        let asset = Asset::new(path.to_string());
        let mut bundle = Bundle::new("test".to_string(), 42);
        bundle.add_asset(asset);
        assert_eq!(bundle.assets.len(), 1);
        assert_eq!(bundle.assets[0].path, path);
    }

    #[test]
    fn test_resource_map_add_bundle() {
        let name = "name";
        let size = 42;
        let bundle = Bundle::new(name.to_string(), size);
        let mut resource_map = ResourceMap::new();
        resource_map.add_bundle(bundle);
        assert_eq!(resource_map.bundles.len(), 1);
        assert_eq!(resource_map.bundles[0].name, name);
        assert_eq!(resource_map.bundles[0].size, size);
    }
}