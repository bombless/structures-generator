use cfg::load_config;
use std::old_io::MemReader;

#[test]
fn test() {
	let urls = "urls = [ \"http://example.com/\", \"http://g.cn/\" ]";
	let bytes = urls.to_string().into_bytes();
	let stream = &mut MemReader::new(bytes);
	let urls = load_config(stream).unwrap();
	assert_eq!(urls, vec![ "http://example.com/", "http://g.cn/" ])
}
