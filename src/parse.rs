use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
struct Struct(HashMap<String, Type>);

#[derive(Debug, PartialEq, Clone)]
struct Union(HashMap<String, Type>);

#[derive(Debug, PartialEq, Clone)]
enum Type {
	Struct(Struct),
	Union(Union),
	DWORD,
	WORD,
	BYTE,
	Pointer(Rc<Type>),
	Unknown(TypeName)
}

fn make_pointer(v: Type)->Type {
	Type::Pointer(Rc::new(v))
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TypeName {
	Normal(String),
	Union(String),
	Struct(String)
}


// I kinda want to use `Iterator<Item=char>` directly but a bug's there freaking me
// so this is a wordaround
trait ReadChar {
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

impl Token {
	fn parse(reader: &mut ReadChar)->Result<Vec<Token>, String> {
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
struct TokenStream(Vec<Token>);

impl TokenStream {
	fn peek(&mut self)->Option<Token> {
		if self.0.is_empty() {
			None
		} else {
			Some(self.0[0].clone())
		}
	}
	
	fn read(&mut self)->Option<Token> {
		if self.0.is_empty() {
			None
		} else {
			Some(self.0.remove(0))
		}
	}
	
	fn new(v: Vec<Token>)->TokenStream {
		TokenStream(v)
	}
	
	fn eat(&mut self, tok: Token)->Result<(), String> {
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

pub fn remove_single_line_comment(code: &str)->String {
	let mut iter = code.chars();
	let mut ret = String::new();
	while let Some(c) = iter.next() {
		match c {
			'/' =>match iter.next() {
				Some('/') =>while let Some(c) = iter.next() {
					if c == '\n' {
						ret.push(c);
						break
					}
				},
				Some(x) =>ret.push(x),
				None =>()
			},
			_ =>ret.push(c)
		}
	}
	ret
}

fn parse_struct(reader: &mut TokenStream)->Result<Struct, String> {
	try!(reader.eat(Token::LeftBrace));
	let mut ret = HashMap::new();
	loop {
		let token = match reader.read() {
			None =>return Err(format!("unexpected EOF")),
			Some(x) =>x
		};
		let val = match token {
			Token::RightBrace =>break,
			Token::Struct =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				}
				Type::Struct(try!(parse_struct(reader)))
			},
			Token::Union =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				}
				Type::Union(try!(parse_union(reader)))
			},
			Token::Ident(name) =>Type::Unknown(TypeName::Normal(name)),
			Token::DWORD =>Type::DWORD,
			Token::WORD =>Type::WORD,
			Token::BYTE =>Type::BYTE,
			Token::SemiColon | Token::Comma | Token::LeftBrace | Token::Typedef | Token::Pointer =>{
				return Err(format!("unexpected token {:?}", token))
			}
		};
		let mut val = val;
		loop {
			let peek = reader.peek();
			if Some(Token::Pointer) == peek {
				reader.read().unwrap();
				val = make_pointer(val);
			} else {
				break
			}
		}
		let val = val;
		match reader.read() {
			Some(Token::Ident(name)) =>if ret.insert(name.clone(), val).is_some() {
				return Err(format!("dup of field name {}", name))
			},
			Some(tok) =>return Err(format!("unexpected token {:?}", tok)),
			None =>return Err(format!("unexpected EOF"))
		}
		try!(reader.eat(Token::SemiColon))
	}
	if ret.is_empty() {
		Err(format!("a struct needs at least one field"))
	} else {
		Ok(Struct(ret))
	}
}

fn parse_union(reader: &mut TokenStream)->Result<Union, String> {
	try!(reader.eat(Token::LeftBrace));
	let mut ret = HashMap::new();
	loop {
		let token = match reader.read() {
			None =>return Err(format!("unexpected EOF")),
			Some(x) =>x
		};
		let val = match token {
			Token::RightBrace =>break,
			Token::Struct =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				}
				Type::Struct(try!(parse_struct(reader)))
			},
			Token::Union =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				}
				Type::Union(try!(parse_union(reader)))
			},
			Token::Ident(name) =>Type::Unknown(TypeName::Normal(name)),
			Token::DWORD =>Type::DWORD,
			Token::WORD =>Type::WORD,
			Token::BYTE =>Type::BYTE,
			Token::SemiColon | Token::Comma | Token::LeftBrace | Token::Typedef | Token::Pointer =>{
				return Err(format!("unexpected token {:?}", token))
			}
		};
		let mut val = val;
		loop {
			let peek = reader.peek();
			if Some(Token::Pointer) == peek {
				reader.read().unwrap();
				val = make_pointer(val);
			} else {
				break
			}
		}
		let val = val;
		match reader.read() {
			Some(Token::Ident(name)) =>if ret.insert(name.clone(), val).is_some() {
				return Err(format!("dup of field name {}", name))
			},
			Some(tok) =>return Err(format!("unexpected token {:?}", tok)),
			None =>return Err(format!("unexpected EOF"))
		}
		try!(reader.eat(Token::SemiColon))
	}
	if ret.is_empty() {
		Err(format!("a union needs at least one field"))
	} else {
		Ok(Union(ret))
	}
}

	
fn parse_typedef(reader: &mut TokenStream)->Result<HashMap<TypeName, Type>, String> {
	let token = match reader.read() {
		None =>return Err(format!("unexpected EOF")),
		Some(x) =>x
	};
	let mut optional_name = None;
	let val = match token {
		Token::Struct =>{
			let peek = reader.peek();
			if let Some(Token::Ident(name)) = peek {
				reader.read().unwrap();
				optional_name = Some(TypeName::Struct(name))
			}
			Type::Struct(try!(parse_struct(reader)))
		},
		Token::Union =>{
			let peek = reader.peek();
			if let Some(Token::Ident(name)) = peek {
				reader.read().unwrap();
				optional_name = Some(TypeName::Union(name))
			}
			Type::Union(try!(parse_union(reader)))
		},
		Token::DWORD =>Type::DWORD,
		Token::WORD =>Type::WORD,
		Token::BYTE =>Type::BYTE,
		Token::SemiColon | Token::Comma | Token::LeftBrace | Token::RightBrace |
			Token::Typedef | Token::Pointer =>{
			return Err(format!("unexpected token {:?}", token))
		},
		Token::Ident(name) =>Type::Unknown(TypeName::Normal(name))
	};
	let mut ret = HashMap::new();
	if let Some(name) = optional_name {
		assert_eq!(ret.insert(name, val.clone()), None)
	}
	loop {
		let mut val = val.clone();
		loop {
			let peek = reader.peek();
			if peek == Some(Token::Pointer) {
				reader.read().unwrap();
				val = make_pointer(val);
			} else {
				break
			}
		}
		let val = val;
		match reader.read() {
			Some(Token::Ident(name)) =>{
				if ret.insert(TypeName::Normal(name.clone()), val).is_some() {
					return Err(format!("dup of typedef name {}", name))
				}
			},
			Some(x) =>return Err(format!("unexpected token {:?}", x)),
			None =>return Err(format!("unexpected EOF"))
		}
		match reader.peek() {
			Some(Token::Comma) =>(),
			Some(Token::SemiColon) =>return Ok(ret),
			Some(x) =>return Err(format!("unexpected token {:?}", x)),
			None =>return Err(format!("unexpected EOF"))
		}
		reader.read().unwrap();
	}
}

#[allow(unstable)]
pub fn compile(reader: &mut ReadChar)->Result<HashMap<TypeName, Type>, String> {
	let tokens = try!(Token::parse(reader));
	let stream = &mut TokenStream::new(tokens);
	let mut ret = HashMap::new();
	loop {
		let token = match stream.read() {
			Some(x) =>x,
			None =>break
		};
		match token {
			Token::Typedef =>for (k, v) in try!(parse_typedef(stream)).drain() {
				if ret.insert(k.clone(), v).is_some() {
					return Err(format!("dup of type name {:?}", k))
				}
			},
			Token::Struct =>{
				let peek = stream.peek();
				if let Some(Token::Ident(name)) = peek {
					stream.read().unwrap();
					if Some(Token::SemiColon) == stream.peek() {
						let struct_name = TypeName::Struct(name);
						let val = Type::Unknown(struct_name.clone());
						let old = ret.insert(struct_name.clone(), val.clone());
						match old {
							Some(Type::Struct(_)) =>{
								ret.insert(struct_name, old.unwrap()).unwrap();
							},
							Some(x) =>if x != val {
								panic!("internal error: type {:?} inside {:?}", x, struct_name)
							},
							None =>()
						}
					} else {
						let struct_name = TypeName::Struct(name);
						let val = Type::Struct(try!(parse_struct(stream)));
						match ret.insert(struct_name.clone(), val.clone()) {
							None | Some(Type::Unknown(TypeName::Struct(_))) =>(),
							x @ Some(Type::Struct(_)) =>if x != Some(val.clone()) {
								return Err(format!(
									"conflict definition of {:?}, new: {:?}, old: {:?}",
									struct_name, val, x.unwrap()))
							},
							x =>panic!("internal error: type {:?} inside {:?}", x, struct_name)
						}
					}
				} else {
					try!(parse_struct(stream));
				}
			},
			Token::Union =>{
				let peek = stream.peek();
				if let Some(Token::Ident(name)) = peek {
					stream.read().unwrap();
					if Some(Token::SemiColon) == stream.peek() {
						let union_name = TypeName::Union(name);
						let val = Type::Unknown(union_name.clone());
						let old = ret.insert(union_name.clone(), val.clone());
						match old {
							Some(Type::Union(_)) =>{
								ret.insert(union_name, old.unwrap()).unwrap();
							},
							Some(x) =>if x != val {
								panic!("internal error: type {:?} inside {:?}", x, union_name)
							},
							None =>()
						}
					} else {
						let union_name = TypeName::Union(name);
						let val = Type::Union(try!(parse_union(stream)));
						match ret.insert(union_name.clone(), val.clone()) {
							None | Some(Type::Unknown(TypeName::Union(_))) =>(),
							x @ Some(Type::Union(_)) =>if x != Some(val.clone()) {
								return Err(format!(
									"conflict definition of {:?}, new: {:?}, old: {:?}",
									union_name, val, x.unwrap()))
							},
							x =>panic!("internal error: type {:?} inside {:?}", x, union_name)
						}
					}
				} else {
					try!(parse_union(stream));
				}
			},
			Token::SemiColon | Token::Comma | Token::LeftBrace | Token::RightBrace |
				Token::Pointer | Token::Ident(_) | Token::DWORD | Token::WORD | Token::BYTE =>{
				return Err(format!("unexpected token {:?}", token))
			}
		}
		try!(stream.eat(Token::SemiColon))
	}
	assert_eq!(stream.peek(), None);
	Ok(ret)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_comment() {
		use parse::remove_single_line_comment;
		assert_eq!(&*remove_single_line_comment("//"), "")
	}

	#[test]
	fn test_simple_case_for_tokenizer() {
		use parse::Token;
		let mut test = format!("typedef DWORD u32;");
		let rslt = Token::parse(&mut test).unwrap();
		assert_eq!(rslt,
			vec![
				Token::Typedef,
				Token::DWORD,
				Token::Ident(format!("u32")),
				Token::SemiColon])
	}
	
	#[test]
	#[allow(unstable)]
	fn test_simple_case_for_parser() {
		use std::collections::HashMap;
		use parse::{
			TypeName,
			Type,
			compile,
			Struct
		};
		let mut tests = vec![
			(
				"typedef DWORD u32;",
				{
					let mut map = HashMap::new();
					map.insert(TypeName::Normal(format!("u32")), Type::DWORD);
					map
				}
			),
			(
				"struct { BYTE b; };", HashMap::new()
			),
			(
				"struct s { BYTE b; };",
				{
					let mut tree = HashMap::new();
					let mut s = HashMap::new();
					s.insert(format!("b"), Type::BYTE);
					let s = Type::Struct(Struct(s));
					tree.insert(TypeName::Struct(format!("s")), s);
					tree
				}
			)
		];
		for (s, m) in tests.drain() {
			assert_eq!(compile(&mut format!("{}", s)).unwrap(), m)
		}
	}
}

