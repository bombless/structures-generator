use std::fs::{create_dir, File};
use std::io::{ErrorKind, Read};
#[cfg(not(test))]
use std::io::Write;
#[cfg(not(test))]
use hyper::Url;
#[cfg(not(test))]
use hyper::client::Client;
use regex::Regex;

#[cfg(test)]
mod tests;

#[cfg(not(test))]
pub struct Page {
	pub url: String,
	pub content: String
}

#[cfg(not(test))]
impl Page {
	pub fn new<'a>(url: &'a str, cnt: &'a str)->Page {
		Page { url: format!("{}", url), content: format!("{}", cnt) }
	}
}

fn cache_path(name: &str)->String {
	if let Err(err) = create_dir(".cache") {
		if err.kind() != ErrorKind::PathAlreadyExists {
			panic!(err)
		}
	}
	name.chars().fold(format!(".cache/"), |mut acc, c| {
		acc.push_str(&format!("{}", c as u64));
		acc
	})
}

fn load_from_cache(url: &str)->Option<String> {
	let path = cache_path(url);
	let mut file = try_or_none!(File::open(&path));
	let mut ret = String::new();
	try_or_none!(file.read_to_string(&mut ret));
	Some(ret)
}

#[cfg(not(test))]
fn load_url(name: &str)->Result<String, String> {
	let url = try_or_str!(Url::parse(name));
	let mut res = try_or_str!(Client::new().get(url).send());
	let body = try_or_str!(res.read_to_string());
	let file_path = cache_path(name);
	try_or_str!(File::create(&file_path)).write_all(&body.clone().into_bytes()).unwrap();
	Ok(body)
}

#[cfg(not(test))]
pub fn fetch_contents(urls: &Vec<String>)->Result<Vec<Page>, String> {
	let mut fail = None;
	let ret = urls.iter().fold(Vec::new(), |mut acc, url| {
		match load_from_cache(url) {
			Some(x) =>acc.push(Page::new(url, &x)),
			None =>match load_url(url) {
				Ok(x) =>acc.push(Page::new(url, &x)),
				Err(x) =>fail = Some(format!("{:?}", x))
			}
		}
		acc
	});
	match fail {
		Some(fail) =>Err(fail),
		None =>Ok(ret)
	}
}

pub fn find_code_blocks(html: &str)->Vec<String> {
	let re = Regex::new(r"<pre>[\s\S]*?</pre>").unwrap();
	re.find_iter(html).fold(Vec::new(), |mut acc, i| {
		let (s, e) = i;
		acc.push(html[s .. e].to_string());
		acc
	})
}

pub fn decode(page: &str)->Result<String, String> {
	let mut entity = None;
	let mut inside_tag = false;
	let mut ret = String::new();
	for c in page.chars() {
		if let Some(s) = entity.clone() {
			if c != ';' {
				entity = Some(format!("{}{}", s, c))
			} else {
				ret.push(match &*s {
					"lt" =>'<',
					"gt" =>'>',
					"amp" =>'&',
					_ =>return Err(format!("unknown entity &{};", s))
				});
				entity = None
			}
		} else if inside_tag {
			match c {
				'>' =>inside_tag = false,
				'<' =>return Err(String::from_str("tag mismatch")),
				'&' =>entity = Some(String::new()),
				_ =>()
			}
		} else {
			match c {
				'&' =>entity = Some(String::new()),
				'<' =>inside_tag = true,
				'\u{a0}' =>ret.push(' '),
				_ =>ret.push(c)
			}
		}
	}
	if let Some(s) = entity {
		Err(format!("unclosed entity {}", s))
	} else if inside_tag {
		Err(String::from_str("unclosed tag"))
	} else {
		Ok(ret)
	}
}
