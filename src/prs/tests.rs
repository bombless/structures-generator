#[test]
#[allow(unstable)]
fn test_simple_case_for_parser() {
	use prs::{
		TypeName,
		Type,
		compile,
		Struct,
		Union,
		GlobalNameSpace
	};
	let mut tests = vec![
		(
			"typedef DWORD u32;",
			{
				let mut ns = GlobalNameSpace::new();
				ns.insert(TypeName::Normal(format!("u32")), Type::Primitive(4));
				ns
			}
		),
		(
			"struct { BYTE b; };", GlobalNameSpace::new()
		),
		(
			"struct s { BYTE b; };",
			{
				let mut ns = GlobalNameSpace::new();
				let mut s = Struct::new();
				s.insert(format!("b"), 1);
				let s = Type::Struct(s);
				ns.insert(TypeName::Struct(format!("s")), s);
				ns
			}
		),
		(
			"typedef struct _s { DWORD val; } s;",
			{
				let mut structure = Struct::new();
				structure.insert(format!("val"), 4);
				let mut ns = GlobalNameSpace::new();
				ns.insert(TypeName::Struct(format!("_s")), Type::Struct(structure.clone()));
				ns.insert(TypeName::Normal(format!("s")), Type::Struct(structure));
				ns
			}
		),
		(
			"typedef struct { union { DWORD val; WORD word; }; } s;",
			{
				let mut u = Union::new();
				u.insert(format!("val"), 4);
				u.insert(format!("word"), 2);
				let mut s = Struct::new();
				assert_eq!(s.inject_union(u), Ok(()));
				let mut ns = GlobalNameSpace::new();
				ns.insert(TypeName::Normal(format!("s")), Type::Struct(s));
				ns
			}
		)
	];
	for (s, m) in tests.drain() {
		assert_eq!(compile(&mut format!("{}", s)).unwrap(), m)
	}
}

	#[test]
fn test_parser_output() {
	use prs::compile;
	vec![
		(
			format!("typedef struct {{\n\t{:32} word;\n\t{:32} val;\n}} s;\n",
						"00 - 02", "00 - 04"),
			compile(&mut format!("{}",
					"typedef struct { union { DWORD val; WORD word; }; } s;")).unwrap()
		)
	].drain().fold((), |_, (lhs, rhs)| assert_eq!(lhs, format!("{:?}", rhs)))
}
