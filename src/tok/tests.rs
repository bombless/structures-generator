#[test]
fn test_simple_case_for_tokenizer() {
	use tok::Token;
	let mut test = "typedef DWORD u32;".chars();
	let rslt = Token::parse(&mut test).unwrap();
	assert_eq!(rslt,
		vec![
			Token::Typedef,
			Token::DWORD,
			Token::Ident(format!("u32")),
			Token::SemiColon])
}
