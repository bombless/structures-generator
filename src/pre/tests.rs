#[test]
fn test_comment() {
	use super::remove_single_line_comments as remove_comments;
	assert_eq!(remove_comments("//"), "")
}
