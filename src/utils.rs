struct Try<T>(Option<T>);

impl<T> Try<T> {
	pub fn try<Output, F: Fn(T)->Option<Output>>(self, f: F)->Try<Output> {
		match self {
			Try(Some(v)) =>Try(f(v)),
			Try(None) =>Try(None)
		}
	}
	pub fn try_or_err<E, Output, F: Fn(T)->Option<Output>>(self, f: F, err: E)->Result<Output, E> {
		match self.try(f).get() {
			Some(v) =>Ok(v),
			None =>Err(err)
		}
	}
	pub fn get(self)->Option<T> {
		self.0
	}
}

pub fn try<T>(v: Option<T>)->Try<T> {
	Try(v)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
		let x = super::try(Some("hello".to_string())).try(|mut x| {
			x.push_str(", world.");
			Some(x)
		});
		assert_eq!("hello, world.".to_string(), x.get().unwrap())
	}
}