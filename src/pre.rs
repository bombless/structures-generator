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


#[cfg(test)]
mod tests {
	#[test]
	fn test_comment() {
		use pre::remove_single_line_comments as remove_comments;
		assert_eq!(&*remove_comments("//"), "")
	}
}
