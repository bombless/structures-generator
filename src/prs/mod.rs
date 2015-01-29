use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::rc::Rc;
use std::fmt::Result as FmtResult;
use std::fmt::{
	Display,
	Debug,
	Formatter
};
use tok::{
	TokenStream,
	Token,
	ReadChar
};

#[cfg(test)]
mod tests;

const POINTER_SIZE: usize = 4;

#[derive(PartialEq, Clone)]
struct Struct(HashMap<String, (usize, usize)>);

impl Struct {
	fn iter(&self)->Iter<String, (usize, usize)> {
		self.0.iter()
	}
	
	fn inject_struct(&mut self, s: Struct)->Result<(), String> {
		let bound = self.size();
		for (k, &(offset, size)) in s.iter() {
			if self.layout_mut().insert(k.clone(), (offset + bound, size)).is_some() {
				return Err(format!("dup of field name {}", k))
			}
		}
		Ok(())
	}
	
	fn inject_union(&mut self, u: Union)->Result<(), String> {
		let bound = self.size();
		for (k, &(offset, size)) in u.iter() {
			if self.layout_mut().insert(k.clone(), (offset + bound, size)).is_some() {
				return Err(format!("dup of field name {}", k))
			}
		}
		Ok(())
	}
	
	fn insert(&mut self, name: String, size: usize)->Option<(usize, usize)> {
		let bound = self.size();
		self.layout_mut().insert(name, (bound, size))
	}
	
	fn new()->Struct {
		Struct(HashMap::new())
	}
	
	fn layout_mut(&mut self)->&mut HashMap<String, (usize, usize)> {
		&mut self.0
	}
	
	fn layout(&self)->&HashMap<String, (usize, usize)> {
		&self.0
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

impl Display for Struct {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		let mut slice = self.layout().iter().fold(Vec::new(), |mut acc, (field_name, &(offset, size))| {
			acc.push(((offset, size), format!("\t{:32} {};\n", if size == 0 {
				format!("{:02X} (offset only, size unknown)", offset)
			} else {
				format!("{:02X} - {:02X}", offset, offset + size)
			}, field_name)));
			acc
		});
		slice.sort();
		write!(f, "{}", slice.drain().fold(String::new(), |acc, (_, s)| {
			acc + &*s
		}))
	}
}

impl Debug for Struct {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		write!(f, "struct {{\n{}}}", self)
	}
}

#[derive(PartialEq, Clone)]
struct Union(HashMap<String, (usize, usize)>);

impl Union {
	fn iter(&self)->Iter<String, (usize, usize)> {
		self.0.iter()
	}
	
	fn inject_struct(&mut self, s: Struct)->Result<(), String> {
		for (k, &v) in s.iter() {
			if self.layout_mut().insert(k.clone(), v).is_some() {
				return Err(format!("dup of field name {}", k))
			}
		}
		Ok(())
	}
	
	fn inject_union(&mut self, u: Union)->Result<(), String> {
		for (k, &v) in u.iter() {
			if self.layout_mut().insert(k.clone(), v).is_some() {
				return Err(format!("dup of field name {}", k))
			}
		}
		Ok(())
	}

	fn insert(&mut self, name: String, size: usize)->Option<(usize, usize)> {
		self.layout_mut().insert(name, (0, size))
	}
	
	fn layout_mut(&mut self)->&mut HashMap<String, (usize, usize)> {
		&mut self.0
	}
	
	fn layout(&self)->&HashMap<String, (usize, usize)> {
		&self.0
	}
	
	fn is_empty(&self)->bool {
		self.layout().is_empty()
	}
	
	fn new()->Union {
		Union(HashMap::new())
	}
	
	fn size(&self)->usize {
		let mut ret = 0;
		for (_, &(offset, size)) in self.layout().iter() {
			if ret < offset + size { ret = offset + size }
		}
		ret
	}
}

impl Display for Union {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		let mut slice = self.layout().iter().fold(Vec::new(), |mut acc, (field_name, &(offset, size))| {
			acc.push(((offset, size), format!("\t{:32} {};\n", if size == 0 {
				format!("{:02X} (offset only, size unknown)", offset)
			} else {
				format!("{:02X} - {:02X}", offset, offset + size)
			}, field_name)));
			acc
		});
		slice.sort();
		write!(f, "{}", slice.drain().fold(String::new(), |acc, (_, s)| {
			acc + &*s
		}))
	}
}

impl Debug for Union {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		write!(f, "union {{\n{}}}", self)
	}
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum TypeName {
	Normal(String),
	Struct(String),
	Union(String)
}

impl Display for TypeName {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		write!(f, "{}", match self {
			&TypeName::Normal(ref s) =>s.clone(),
			&TypeName::Struct(ref s) =>format!("{}", s.clone()),
			&TypeName::Union(ref s)=>format!("{}", s.clone())
		})
	}
}

#[derive(Clone, PartialEq)]
enum Type {
	Struct(Struct),
	Union(Union),
	Primitive(usize),
	Pointer(Rc<Type>),
	Unknown(TypeName)
}

impl Debug for Type {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		write!(f, "{}", match self {
			&Type::Struct(ref s) =>format!("{:?}", s),
			&Type::Union(ref u) =>format!("{:?}", u),
			&Type::Primitive(size) =>format!("primitive[size: {}]", size),
			&Type::Pointer(ref rc) =>format!("{:?}*", &*rc),
			&Type::Unknown(ref name) =>format!("{}", name)
		})
	}
}

#[derive(Clone, PartialEq)]
struct GlobalNameSpace(HashMap<TypeName, Type>);

impl GlobalNameSpace {
	fn new()->GlobalNameSpace {
		GlobalNameSpace(HashMap::new())
	}
	
	fn insert(&mut self, k: TypeName, v: Type)->Option<Type> {
		self.0.insert(k, v)
	}
	
	fn iter(&self)->Iter<TypeName, Type> {
		self.0.iter()
	}
	
	fn drain(self)->(GlobalNameSpace, ) {
		(self,)
	}
	
	fn remove(&mut self, v: &TypeName)->Option<Type> {
		self.0.remove(v)
	}
}

impl Debug for GlobalNameSpace {
	fn fmt(&self, f: &mut Formatter)->FmtResult {
		for (k, v) in self.iter() {
			match k {
				&TypeName::Normal(ref name) =>try!(write!(f, "typedef {:?} {};\n", v, name)),
				&TypeName::Struct(ref name) =>if let &Type::Struct(ref s) = v {
					try!(write!(f, "struct {} {{\n{}}};\n", name, s))
				} else {
					panic!("internal error, {:?} inside {}", v, name)
				},
				&TypeName::Union(ref name) =>if let &Type::Union(ref s) = v {
					try!(write!(f, "union {} {{\n{}}};\n", name, s))
				} else {
					panic!("internal error, {:?} inside {}", v, name)
				}
			}
		}
		Ok(())
	}
}
// Maybe better to impl iter for (GlobalNameSpace, ) and impl drain for GlobalNameSpace .. next time

impl Iterator for (GlobalNameSpace, ) {
	type Item = (TypeName, Type);
	fn next(&mut self)->Option<(TypeName, Type)> {
		let clone = self.0.clone();
		if let Some((k, v)) = clone.iter().next() {
			let mut clone = self.0.clone();
			assert!(clone.remove(k).is_some());
			self.0 = clone;
			Some((k.clone(), v.clone()))
		} else {
			None
		}
	}
}

fn make_pointer(v: Type)->Type {
	Type::Pointer(Rc::new(v))
}

fn parse_struct(reader: &mut TokenStream)->Result<Struct, String> {
	try!(reader.eat(Token::LeftBrace));
	let mut ret = Struct::new();
	loop {
		let token = match reader.read() {
			None =>return Err(format!("unexpected EOF")),
			Some(x) =>x
		};
		let size = match token {
			Token::RightBrace =>break,
			Token::Struct =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				}
				let s = try!(parse_struct(reader));
				let peek = reader.peek();
				if peek == Some(Token::SemiColon) {
					reader.read().unwrap();
					try!(ret.inject_struct(s));
					continue
				} else {
					s.size()
				}
			},
			Token::Union =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				}
				let u = try!(parse_union(reader));
				let peek = reader.peek();
				if peek == Some(Token::SemiColon) {
					reader.read().unwrap();
					try!(ret.inject_union(u));
					continue
				} else {
					u.size()
				}
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

fn parse_union(reader: &mut TokenStream)->Result<Union, String> {
	try!(reader.eat(Token::LeftBrace));
	let mut ret = Union::new();
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
				let s = try!(parse_struct(reader));
				let peek = reader.peek();
				if peek == Some(Token::SemiColon) {
					reader.read().unwrap();
					try!(ret.inject_struct(s));
					continue
				} else {
					s.size()
				}
			},
			Token::Union =>{
				let peek = reader.peek();
				if let Some(Token::Ident(_)) = peek {
					reader.read().unwrap();
				};
				let u = try!(parse_union(reader));
				let peek = reader.peek();
				if peek == Some(Token::SemiColon) {
					reader.read().unwrap();
					try!(ret.inject_union(u));
					continue
				} else {
					u.size()
				}
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

	
fn parse_typedef(reader: &mut TokenStream)->Result<GlobalNameSpace, String> {
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
				optional_name = Some(TypeName::Struct(name.clone()))
			}
			Type::Struct(try!(parse_struct(reader)))
		},
		Token::Union =>{
			let peek = reader.peek();
			if let Some(Token::Ident(name)) = peek {
				reader.read().unwrap();
				optional_name = Some(TypeName::Union(name.clone()))
			}
			Type::Union(try!(parse_union(reader)))
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
	let mut ret = GlobalNameSpace::new();
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

pub fn compile(reader: &mut ReadChar)->Result<GlobalNameSpace, String> {
	let tokens = try!(Token::parse(reader));
	let stream = &mut TokenStream::new(tokens);
	let mut ret = GlobalNameSpace::new();
	loop {
		let token = match stream.read() {
			Some(x) =>x,
			None =>break
		};
		match token {
			Token::Typedef =>for (k, v) in try!(parse_typedef(stream)).drain() {
				if ret.insert(k.clone(), v).is_some() {
					return Err(format!("dup of type name {}", k))
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
								panic!("internal error: type {:?} inside {}", x, struct_name)
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
									"conflict definition of {}, new: {:?}, old: {:?}",
									struct_name, val, x.unwrap()))
							},
							x =>panic!("internal error: type {:?} inside {}", x, struct_name)
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
								panic!("internal error: type {:?} inside {}", x, union_name)
							},
							None =>()
						}
					} else {
						let union_name = TypeName::Union(name.clone());
						let val = Type::Union(try!(parse_union(stream)));
						match ret.insert(union_name.clone(), val.clone()) {
							None | Some(Type::Unknown(TypeName::Union(_))) =>(),
							x @ Some(Type::Union(_)) =>if x != Some(val.clone()) {
								return Err(format!(
									"conflict definition of {}, new: {:?}, old: {:?}",
									union_name, val, x.unwrap()))
							},
							x =>panic!("internal error: type {:?} inside {}", x, union_name)
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
