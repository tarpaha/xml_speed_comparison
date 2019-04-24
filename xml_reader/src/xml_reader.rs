use super::types;
use std::io::BufRead;

pub trait XmlReader {
    fn read<R: BufRead>(data: R) -> types::ResourceMap;
}