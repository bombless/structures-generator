#[test]
fn test_decode() {
	use web::decode;
	let rslt = decode("<a>hi</a>").unwrap();
	assert_eq!(rslt, "hi")
}
#[test]
fn test_path() {
	use web::cache_path;
	let path = cache_path(" ");
	assert_eq!(path, ".cache/32")
}
#[test]
fn find_code_blocks() {
	use web::find_code_blocks;
	let rslt = find_code_blocks("");
	assert_eq!(rslt.len(), 0)
}
#[test]
fn test_cache() {
	use super::{
		load_from_cache,
		cache_path
	};
	use std::fs::File;
	use std::io::Write;
	let path = cache_path("test");
	File::create(&path).unwrap().write_all(b"test").unwrap();
	let rslt = load_from_cache("test").unwrap();
	assert_eq!(rslt, "test")
}
