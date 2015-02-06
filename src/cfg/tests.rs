use cfg::load_config;
use std::old_io::MemReader;

#[test]
fn test() {
	let bytes = "[ \"http://example.com/\" ]".to_string().into_bytes();
	let stream = &mut MemReader::new(bytes);
	assert_eq!(load_config(stream).unwrap(), vec!["http://example.com/"])
}
