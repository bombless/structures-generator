use std::old_io::Reader;
use serialize::json::decode;

#[cfg(test)]
mod tests;

pub fn load_config(file: &mut Reader)->Result<Vec<String>, String> {
	let cnt = try_or_str!(file.read_to_string());
	Ok(try_or_str!(decode(&*cnt)))
}
