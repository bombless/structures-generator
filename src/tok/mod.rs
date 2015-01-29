#[cfg(test)]
mod tests;

// I kinda want to use `Iterator<Item=char>` directly but a bug's there freaking me
// so this is a workaround
pub trait ReadChar {
	fn read(&mut self)->Option<char>;
}

impl ReadChar for String {
	fn read(&mut self)->Option<char> {
		if self.is_empty() {
			None
		} else {
			Some(self.remove(0))
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Ident(String),
	Struct,
	Union,
	DWORD,
	WORD,
	BYTE,
	SemiColon,
	Comma,
	LeftBrace,
	RightBrace,
	Typedef,
	Pointer
}

impl Token {
	pub fn parse(reader: &mut ReadChar)->Result<Vec<Token>, String> {
		let mut ret = Vec::new();
		let mut elem = String::new();
		
		macro_rules! append_word{
			() => (
				if !elem.is_empty() {
					let first = elem.as_bytes()[0];
					if '0' as u8 <= first && '9' as u8 >= first {
						return Err(format!("unexpected character {:?}", first as char))
					}
					ret.push(match &*elem {
						"struct" =>Token::Struct,
						"union" =>Token::Union,
						"DWORD" =>Token::DWORD,
						"WORD" =>Token::WORD,
						"BYTE" =>Token::BYTE,
						"typedef" =>Token::Typedef,
						_ =>Token::Ident(elem.clone())
					});
					elem.clear()
				}
			)
		}
				
		while let Some(c) = reader.read() {
			match c {
				' ' | '\t' | '\r' | '\n' =>{
					append_word!();
				},
				'*' =>{
					append_word!();
					ret.push(Token::Pointer)
				},
				';' =>{
					append_word!();
					ret.push(Token::SemiColon)
				},
				',' =>{
					append_word!();
					ret.push(Token::Comma)
				},
				'{' =>{
					append_word!();
					ret.push(Token::LeftBrace)
				},
				'}' =>{
					append_word!();
					ret.push(Token::RightBrace)
				},
				'_' | '0' ... '9' | 'a' ... 'z' | 'A' ... 'Z' =>elem.push(c),
				_ =>return Err(format!("unexpected character {:?}", c))
			}
		}
		append_word!();
		Ok(ret)		
	}
}

#[derive(Clone, Debug)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
	pub fn peek(&mut self)->Option<Token> {
		if self.0.is_empty() {
			None
		} else {
			Some(self.0[0].clone())
		}
	}
	
	pub fn read(&mut self)->Option<Token> {
		if self.0.is_empty() {
			None
		} else {
			Some(self.0.remove(0))
		}
	}
	
	pub fn new(v: Vec<Token>)->TokenStream {
		TokenStream(v)
	}
	
	pub fn eat(&mut self, tok: Token)->Result<(), String> {
		match self.read() {
			None =>Err(format!("unexpected EOF")),
			Some(x) =>if tok != x {
				Err(format!("unexpected token {:?}", x))
			} else {
				Ok(())
			}
		}
	}
}
