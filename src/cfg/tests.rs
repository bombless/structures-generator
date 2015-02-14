#[test]
fn test() {
	let mut input = b"urls = [ \"http://example.com/\", \"http://g.cn/\" ]";
	let urls = super::load_config(&mut input).unwrap();
	assert_eq!(urls, vec![ "http://example.com/", "http://g.cn/" ])
}
