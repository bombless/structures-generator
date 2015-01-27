macro_rules! try_or_str {
	($v:expr) => (
		match $v {
			Ok(x) =>x,
			Err(e)=>return Err(format!("{:?}", e))
		}
	)
}

macro_rules! try_or_none {
	($v:expr) => (
		match $v {
			Ok(x) =>x,
			Err(_)=>return None
		}
	)
}
