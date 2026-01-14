/* 2026 - @furry_onko */
use std::env;

mod argv;
mod visual;

fn main() {
	let file = argv::proc(env::args());

	println!("{:?}", &file);
/*	match file {
		Ok(f_sum) => {
			println!("File: {:?}", f_sum.path);
			println!("Mode: {:?}", f_sum.mode);
		},
		Err(f_err) => {
			if f_err == argv::ProcErrors::FileNotSpecified {
				visual::error("Given file is not specified.");
			}
			else { unreachable!() }
		}
	}*/
}