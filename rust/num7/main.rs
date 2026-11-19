#[path = "../structs/mod.rs"]
mod structs;

use crate::structs::lru_cache::LRUCache;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("failed to read input");
        return;
    }

    let mut tokens = input.split_whitespace();

    let cap_str = match tokens.next() {
        Some(s) => s,
        None => return,
    };
    let cap: usize = match cap_str.parse() {
        Ok(c) => c,
        Err(_) => return,
    };

    let q_str = match tokens.next() {
        Some(s) => s,
        None => return,
    };
    let q: usize = match q_str.parse() {
        Ok(q) => q,
        Err(_) => return,
    };

    let mut cache = LRUCache::new(cap);

    for _ in 0..q {
        let op = match tokens.next() {
            Some(op) => op,
            None => break,
        };

        if op == "SET" {
            let x: i64 = tokens.next().unwrap().parse().unwrap();
            let y: i64 = tokens.next().unwrap().parse().unwrap();
            cache.set(x, y);
        } else if op == "GET" {
            let x: i64 = tokens.next().unwrap().parse().unwrap();
            
            let val_ptr = cache.get(&x);
            
            if !val_ptr.is_null() {
                unsafe {
                    println!("{}", *val_ptr);
                }
            } else {
                println!("-1");
            }
        }
    }
}