use std::path::Path;
use std::old_io::fs::File;
use serialize::json::decode;

#[cfg(test)]
mod tests;

pub fn load_config()->Result<Vec<String>, String> {
	let path = Path::new("urls.json");
	let mut file = try_or_str!(File::open(&path));
	let cnt = try_or_str!(file.read_to_string());
	Ok(try_or_str!(decode(&*cnt)))
}
