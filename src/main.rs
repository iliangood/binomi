use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;

use rayon::prelude::*;
use std::{
    env,
    io::{self, Write},
};

struct MemFactorial {
    facts: Vec<BigInt>,
}
impl MemFactorial {
    fn new() -> MemFactorial {
        MemFactorial {
            facts: vec![BigInt::from(1), BigInt::from(1)],
        }
    }
    fn calc(&mut self, num: usize) -> BigInt {
        if self.facts.len() > num {
            return self.facts[num].clone();
        }
        for i in self.facts.len()..=num {
            self.facts
                .push(self.facts.last().unwrap().clone() * BigInt::from(i));
        }
        self.facts.last().unwrap().clone()
    }
}

fn multi_factorial(num: usize) -> BigInt {
    if num < 2 {
        return BigInt::from(1);
    }
    (2..=num)
        .into_par_iter()
        .map(BigInt::from)
        .reduce(|| BigInt::from(1), |acc, cur| acc * cur)
}

struct MemCombinations {
    fact_calc: MemFactorial,
}
impl MemCombinations {
    fn new() -> MemCombinations {
        MemCombinations {
            fact_calc: MemFactorial::new(),
        }
    }
    fn calc(&mut self, base: usize, subset: usize) -> BigInt {
        if base < 100000 && subset < 100000 {
            self.fact_calc.calc(base)
                / self.fact_calc.calc(base - subset)
                / self.fact_calc.calc(subset)
        } else {
            multi_factorial(base) / multi_factorial(base - subset) / multi_factorial(subset)
        }
    }
}

// fn combinations(base: usize, subset: usize) -> BigInt {
//     multi_factorial(base) / multi_factorial(base - subset) / multi_factorial(subset)
// }
fn main() {
    let mut use_float = false;
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-f" => use_float = true,
            _ => {
                println!("unkown argument: {}", arg);
            }
        }
    }
    let mut mem_comb = MemCombinations::new();
    let mut input = String::new();
    let n = loop {
        print!("enter n:");
        io::stdout().flush().expect("output error");
        io::stdin().read_line(&mut input).expect("input error");
        let num_s = input.trim();
        match num_s.parse::<usize>() {
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
        io::stdout().flush().expect("output error");
        io::stdin().read_line(&mut input).expect("input error");
        let num_s = input.trim();
        match num_s.parse::<BigRational>() {
            Err(_) => println!("{} isn't a rational number between 0 and 1", num_s),
            Ok(num) => {
                if num < BigRational::from_integer(BigInt::from(0))
                    || num > BigRational::from_integer(BigInt::from(1))
                {
                    println!("{} isn't a number between 0 and 1", num_s);
                } else {
                    input.clear();
                    break num;
                }
            }
        }
        input.clear();
    };
    let q = BigRational::from_integer(BigInt::from(1)) - &p;
    if use_float {
        for i in 0..=n {
            println!(
                "{}:{}",
                i,
                (BigRational::from(mem_comb.calc(n, i))
                    * p.clone().pow(i as i32)
                    * q.clone().pow((n - i) as i32))
                .to_f64()
                .expect("number is to big to represent it in f64")
            );
        }
    } else {
        for i in 0..=n {
            println!(
                "{}:{}",
                i,
                BigRational::from(mem_comb.calc(n, i))
                    * p.clone().pow(i as i32)
                    * q.clone().pow((n - i) as i32)
            );
        }
    }
}
