use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
// use clap

use rayon::prelude::*;
use std::{
    cmp, env,
    io::{self, Write},
    thread::{self, JoinHandle},
};

struct MemFactorial {
    facts: Option<Vec<BigInt>>,
}
impl MemFactorial {
    fn new() -> MemFactorial {
        MemFactorial {
            facts: Some(vec![BigInt::from(1), BigInt::from(1)]),
        }
    }
    fn calc(&mut self, num: usize) -> Option<BigInt> {
        let mut facts = self
            .facts
            .take()
            .unwrap_or(vec![BigInt::from(1), BigInt::from(1)]);
        if facts.len() > num {
            return std::panic::catch_unwind(|| facts[num].clone()).ok();
        }
        facts.try_reserve(num.saturating_sub(facts.len())).ok()?;
        self.facts = std::panic::catch_unwind(move || {
            for i in facts.len()..=num {
                facts.push(facts.last().unwrap() * BigInt::from(i));
            }
            facts
        })
        .ok();
        if let Some(facts) = &self.facts {
            facts.last().cloned()
        } else {
            None
        }
    }
}

fn _multi_factorial(num: usize) -> BigInt {
    if num < 2 {
        return BigInt::from(1);
    }
    (2..=num).into_par_iter().map(BigInt::from).product()
}

fn simple_factorial(num: usize) -> BigInt {
    (2..=num).map(BigInt::from).product()
}

fn multi_factorial(num: usize) -> Option<BigInt> {
    let cpu_count = num_cpus::get();
    let prefer = cmp::max(num / 256, 1);
    let threads_count = cmp::min(cpu_count, prefer);
    let part = num / threads_count;
    let mut start = 2usize;
    let mut threads = Vec::<JoinHandle<BigInt>>::with_capacity(threads_count);
    for _ in 1..threads_count {
        threads.push(thread::spawn(move || {
            let end = start + part;
            (start..end).map(BigInt::from).product()
        }));
        start += part;
    }
    threads.push(thread::spawn(move || {
        (start..=num).map(BigInt::from).product()
    }));
    // threads.iter_mut().map(|th| th.join().unwrap()).product()
    let mut res = BigInt::from(1);
    for th in threads {
        if let Ok(cur) = th.join() {
            res *= cur;
        } else {
            return None;
        }
    }
    Some(res)
}

struct SmartFactorial {
    mem_factorial: MemFactorial,
    mem_limit: usize,
}

impl SmartFactorial {
    fn new() -> SmartFactorial {
        SmartFactorial {
            mem_factorial: MemFactorial::new(),
            mem_limit: 100_000,
        }
    }
    fn calc(&mut self, num: usize) -> BigInt {
        if num <= self.mem_limit
            && let Some(res) = self.mem_factorial.calc(num)
        {
            return res;
        }
        multi_factorial(num).unwrap_or(simple_factorial(num))
    }
}

struct MemCombinations {
    smart_factorial: SmartFactorial,
}
impl MemCombinations {
    fn new() -> MemCombinations {
        MemCombinations {
            smart_factorial: SmartFactorial::new(),
        }
    }
    fn calc(&mut self, base: usize, subset: usize) -> BigInt {
        self.smart_factorial.calc(base)
            / self.smart_factorial.calc(base - subset)
            / self.smart_factorial.calc(subset)
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
