
extern crate syn;

fn main() {

	let file = r#"#!/usr/bin/env run-cargo-script

	//!
	//!
	//!```cargo
	//!time = "*"
	//!```
	//!
	//!

	#[cargo(version = "*")]
	extern crate time;

	fn main() {
		println!("Hello, World!");
	}
	"#;

	let file = syn::parse_file(file).unwrap();

	println!("{:#?}", file);

}


