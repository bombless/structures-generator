use std::io::Read;
use utils::try;
use toml::Parser;
use toml::Value::{
	String,
	Array
};

#[cfg(test)]
mod tests;

pub fn load_config(file: &mut Read)->Result<Vec<String>, String> {
	let cnt = {
		let mut cnt = String::new();
		try_or_str!(file.read_to_string(&mut cnt));
		try(cnt)
	};
	let tbl = cnt.try(|x| Parser::new(&x).parse());
	tbl.try_or_err(|x| match x.get("urls") {
		Some(&Array(ref arr)) =>{
			let mut ret = Vec::new();
			for item in arr {
				if let &String(ref x) = item {
					ret.push(x.clone())
				} else {
					return None
				}
			}
			Some(ret)
		},
		_ =>None
	}, "illegal format".to_string())
}
