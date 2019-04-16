use quick_xml::{Reader, events::Event};
use std::borrow::Cow;

fn main() {

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

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"Bundle" => {
                        let download_size_chars = e.attributes()
                            .map(|a| a.unwrap())
                            .find(|a| a.key == b"DownloadSize")
                            .unwrap()
                            .value;
                        let download_size = cow_chars_to_u32(&download_size_chars);
                        println!("{}", download_size)
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => ()
        }
        buf.clear();
    }
}

fn cow_chars_to_u32(chars: &Cow<[u8]>) -> u32 {
    let mut result = 0;
    for ch in chars.iter() {
        result *= 10;
        result += (ch - 48) as u32;
    }
    result
} 
