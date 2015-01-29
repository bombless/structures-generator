#[cfg(test)]
mod tests;

pub fn remove_single_line_comments(code: &str)->String {
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
