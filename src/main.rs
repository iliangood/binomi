use num_rational::BigRational;
use num_bigint::BigInt;
use std::io::{self, Write};
fn combinations(base: BigInt, subset: BigInt) -> BigInt {
	base.
}
fn main() {
	let mut input = String::new();
	let n = loop {
		print!("enter n:");
        	io::stdout().flush().ok().expect("output error");
        	io::stdin().read_line(&mut input).ok().expect("input error");	
		let num_s = input.trim();
		match num_s.parse::<u32>() {
			Err(_) => println!("{} isn't a positive number", num_s),
			Ok(0) => println!("{} is zero, number should be positive", 0),
			Ok(num) => { 
				input.clear();
				break num;
			}
		}
		input.clear();
	};
	let p = loop {
		print!("enter p:");
		io::stdout().flush().ok().expect("output error");
		io::stdin().read_line(&mut input).ok().expect("input error");
		let num_s = input.trim();
		match num_s.parse::<BigRational>() {
			Err(_) => println!("{} isn't a rational number between 0 and 1", num_s),
			Ok(num) => {
				if num < BigRational::from_integer(BigInt::from(0)) || num > BigRational::from_integer(BigInt::from(1)) {
					println!("{} isn't a number between 0 and 1", num_s);
				} else {
					input.clear();
					break num;
				}
			}
		}
		input.clear();
	};
	for i in 0..=n {
		
	}
}
