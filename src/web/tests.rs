#[test]
fn test_decode() {
	use web::decode;
	let rslt = decode("<a>hi</a>").unwrap();
	assert_eq!(&*rslt, "hi")
}
#[test]
fn test_path() {
	use web::cache_path;
	let path = cache_path(" ");
	assert_eq!(Path::new(".cache/32"), path)
}
#[test]
fn find_code_blocks() {
	use web::find_code_blocks;
	let rslt = find_code_blocks("");
	assert_eq!(rslt.len(), 0)
}
#[test]
fn test_cache() {
	use web::{
		load_from_cache,
		cache_path
	};
	use std::io::fs::File;
	let path = cache_path("test");
	File::create(&path).write_str("test").unwrap();
	let rslt = load_from_cache("test").unwrap();
	assert_eq!(&*rslt, "test")
}
