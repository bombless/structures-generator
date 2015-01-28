use std::collections::HashMap;
use std::cmp::max;
use std::rc::Rc;
use tok::{
	TokenStream,
	Token,
	ReadChar
};

const POINTER_SIZE: usize = 4;

#[derive(Debug, PartialEq, Clone)]
enum Struct {
	Named {
		name: String,
		layout: HashMap<String, (usize, usize)>
	},
	Unnamed {
		layout: HashMap<String, (usize, usize)>
	}
}

impl Struct {
	fn insert(&mut self, name: String, size: usize)->Option<(usize, usize)> {
		let bound = self.size();
		self.layout_mut().insert(name, (bound, size))
	}
	
	fn new(name: Option<String>)->Struct {
		match name {
			Some(name) =>Struct::Named {
				layout: HashMap::new(),
				name: name
			},
			None =>Struct::Unnamed {
				layout: HashMap::new()
			}
		}
	}
	
	fn layout_mut(&mut self)->&mut HashMap<String, (usize, usize)> {
		match self {
			&mut Struct::Named { ref mut layout, .. } | &mut Struct::Unnamed { ref mut layout } =>{
				layout
			}
		}
	}
	
	fn layout(&self)->&HashMap<String, (usize, usize)> {
		match self {
			&Struct::Named { ref layout, .. } | &Struct::Unnamed { ref layout } =>{
				layout
			}
		}
	}
	
	fn is_empty(&self)->bool {
		self.layout().is_empty()
	}
	
	fn size(&self)->usize {
		let mut ret = 0;
		for (_, &(offset, size)) in self.layout().iter() {
			if ret < offset + size { ret = offset + size }
		}
		ret
	}
}

#[derive(Debug, PartialEq, Clone)]
enum Union {
	Named {
		name: String,
		layout: HashMap<String, usize>
	},
	Unnamed {
		layout: HashMap<String, usize>
	}
}

impl Union {
	fn insert(&mut self, name: String, size: usize)->Option<usize> {
		self.layout_mut().insert(name, size)
	}
	
	fn layout_mut(&mut self)->&mut HashMap<String, usize> {
		match self {
			&mut Union::Named { ref mut layout, .. } | &mut Union::Unnamed { ref mut layout } =>{
				layout
			}
		}
	}
	
	fn layout(&self)->&HashMap<String, usize> {
		match self {
			&Union::Named { ref layout, .. } | &Union::Unnamed { ref layout } =>{
				layout
			}
		}
	}
	
	fn is_empty(&self)->bool {
		self.layout().is_empty()
	}
	
	fn new(name: Option<String>)->Union {
		match name {
			Some(name) =>Union::Named {
				layout: HashMap::new(),
				name: name
			},
			None =>Union::Unnamed {
				layout: HashMap::new()
			}
		}
	}
	
	fn size(&self)->usize {
		self.layout().iter().fold(0, |acc, (_, &size)| max(acc, size))
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TypeName {
	Normal(String),
	Union(String),
	Struct(String)
}

#[derive(Clone, PartialEq, Debug)]
enum Type {
	Struct(Struct),
	Union(Union),
	Primitive(usize),
	Pointer(Rc<Type>),
	Unknown(TypeName)
}

fn make_pointer(v: Type)->Type {
	Type::Pointer(Rc::new(v))
}

fn parse_struct(reader: &mut TokenStream, name: Option<String>)->Result<Struct, String> {
	try!(reader.eat(Token::LeftBrace));
	let mut ret = Struct::new(name);
	loop {
		let token = match reader.read() {
			None =>return Err(format!("unexpected EOF")),
			Some(x) =>x
		};
		let size = match token {
			Token::RightBrace =>break,
			Token::Struct =>{
				let peek = reader.peek();
				let name = if let Some(Token::Ident(name)) = peek {
					reader.read().unwrap();
					Some(name)
				} else {
					None
				};
				try!(parse_struct(reader, name)).size()
			},
			Token::Union =>{
				let peek = reader.peek();
				let name = if let Some(Token::Ident(name)) = peek {
					reader.read().unwrap();
					Some(name)
				} else {
					None
				};
				try!(parse_union(reader, name)).size()
			},
			Token::Ident(_) =>0,
			Token::DWORD =>4,
			Token::WORD =>2,
			Token::BYTE =>1,
			Token::SemiColon | Token::Comma | Token::LeftBrace | Token::Typedef | Token::Pointer =>{
				return Err(format!("unexpected token {:?}", token))
			}
		};
		let mut size = size;
		loop {
			let peek = reader.peek();
			if Some(Token::Pointer) == peek {
				reader.read().unwrap();
				size = POINTER_SIZE;
			} else {
				break
			}
		}
		let size = size;
		match reader.read() {
			Some(Token::Ident(name)) =>if ret.insert(name.clone(), size).is_some() {
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
		Ok(ret)
	}
}

fn parse_union(reader: &mut TokenStream, name: Option<String>)->Result<Union, String> {
	try!(reader.eat(Token::LeftBrace));
	let mut ret = Union::new(name);
	loop {
		let token = match reader.read() {
			None =>return Err(format!("unexpected EOF")),
			Some(x) =>x
		};
		let val = match token {
			Token::RightBrace =>break,
			Token::Struct =>{
				let peek = reader.peek();
				let name = if let Some(Token::Ident(name)) = peek {
					reader.read().unwrap();
					Some(name)
				} else {
					None
				};
				try!(parse_struct(reader, name)).size()
			},
			Token::Union =>{
				let peek = reader.peek();
				let name = if let Some(Token::Ident(name)) = peek {
					reader.read().unwrap();
					Some(name)
				} else {
					None
				};
				try!(parse_union(reader, name)).size()
			},
			Token::Ident(_) =>0,
			Token::DWORD =>4,
			Token::WORD =>2,
			Token::BYTE =>1,
			Token::SemiColon | Token::Comma | Token::LeftBrace | Token::Typedef | Token::Pointer =>{
				return Err(format!("unexpected token {:?}", token))
			}
		};
		let mut val = val;
		loop {
			let peek = reader.peek();
			if Some(Token::Pointer) == peek {
				reader.read().unwrap();
				val = POINTER_SIZE;
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
		Ok(ret)
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
			let name = if let Some(Token::Ident(name)) = peek {
				reader.read().unwrap();
				optional_name = Some(TypeName::Struct(name.clone()));
				Some(name)
			} else {
				None
			};
			Type::Struct(try!(parse_struct(reader, name)))
		},
		Token::Union =>{
			let peek = reader.peek();
			let name = if let Some(Token::Ident(name)) = peek {
				reader.read().unwrap();
				optional_name = Some(TypeName::Union(name.clone()));
				Some(name)
			} else {
				None
			};
			Type::Union(try!(parse_union(reader, name)))
		},
		Token::DWORD =>Type::Primitive(4),
		Token::WORD =>Type::Primitive(2),
		Token::BYTE =>Type::Primitive(1),
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
						let struct_name = TypeName::Struct(name.clone());
						let val = Type::Struct(try!(parse_struct(stream, Some(name))));
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
					try!(parse_struct(stream, None));
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
						let union_name = TypeName::Union(name.clone());
						let val = Type::Union(try!(parse_union(stream, Some(name))));
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
					try!(parse_union(stream, None));
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
	#[allow(unstable)]
	fn test_simple_case_for_parser() {
		use std::collections::HashMap;
		use prs::{
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
					map.insert(TypeName::Normal(format!("u32")), Type::Primitive(4));
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
					let mut s = Struct::new(Some(format!("s")));
					s.insert(format!("b"), 1);
					let s = Type::Struct(s);
					tree.insert(TypeName::Struct(format!("s")), s);
					tree
				}
			),
			(
				"typedef struct _s { DWORD val; } s;",
				{
					let mut structure = Struct::new(Some(format!("_s")));
					structure.insert(format!("val"), 4);
					let mut map = HashMap::new();
					map.insert(TypeName::Struct(format!("_s")), Type::Struct(structure.clone()));
					map.insert(TypeName::Normal(format!("s")), Type::Struct(structure));
					map
				}
			)
		];
		for (s, m) in tests.drain() {
			assert_eq!(compile(&mut format!("{}", s)).unwrap(), m)
		}
	}
}

