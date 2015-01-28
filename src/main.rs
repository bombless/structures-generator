extern crate hyper;
#[allow(unstable)]
extern crate serialize;
#[allow(unstable)]
extern crate regex;

#[macro_use]
mod macros;
#[allow(unstable)]
#[cfg(not(test))]
mod cfg;
#[allow(unstable)]
mod web;
#[allow(unstable)]
mod parse;
mod pre;
mod tok;


#[cfg(not(test))]
fn main() {
	let config = cfg::load_config().unwrap();
	for page in web::fetch_contents(&config).unwrap().iter() {
		println!("[{}]", page.url);
		let code_blocks = web::find_code_blocks(&*page.content);
		let cnt = code_blocks.len();
		match cnt {
			0 =>println!("no code blocks here, page size {}", page.content.len()),
			_ =>{
				println!("{} code block(s):", cnt);
				for block in code_blocks.iter() {
					match web::decode(&**block) {
						Ok(code) =>{
							let code = pre::remove_single_line_comments(&*code);
							match parse::compile(&mut code.clone()) {
								Ok(x) =>println!("{:?}", x),
								Err(e) =>println!("error: {}, code<<<{}>>>", e, code)
							}
						},
						Err(e) =>println!("error: {}", e)
					}
				}
			}
		}
	}
}
