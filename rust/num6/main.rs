#[path = "../structs/mod.rs"]
mod structs;

use crate::structs::array_list::ArrayList;
use crate::structs::hash_table::{DoubleHashTable, HashTable};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: ./main <number>");
        process::exit(1);
    }

    // Аналог stoi с обработкой ошибки
    let mut number: i64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("argument is not number");
            process::exit(1);
        }
    };

    let mut dict: DoubleHashTable<String, i64> = DoubleHashTable::new();
    dict.insert("M".to_string(), 1000);
    dict.insert("CM".to_string(), 900);
    dict.insert("D".to_string(), 500);
    dict.insert("CD".to_string(), 400);
    dict.insert("C".to_string(), 100);
    dict.insert("XC".to_string(), 90);
    dict.insert("L".to_string(), 50);
    dict.insert("XL".to_string(), 40);
    dict.insert("X".to_string(), 10);
    dict.insert("IX".to_string(), 9);
    dict.insert("V".to_string(), 5);
    dict.insert("IV".to_string(), 4);
    dict.insert("I".to_string(), 1);

    let mut symbols = ArrayList::new();
    symbols.pushBack("M".to_string());
    symbols.pushBack("CM".to_string());
    symbols.pushBack("D".to_string());
    symbols.pushBack("CD".to_string());
    symbols.pushBack("C".to_string());
    symbols.pushBack("XC".to_string());
    symbols.pushBack("L".to_string());
    symbols.pushBack("XL".to_string());
    symbols.pushBack("X".to_string());
    symbols.pushBack("IX".to_string());
    symbols.pushBack("V".to_string());
    symbols.pushBack("IV".to_string());
    symbols.pushBack("I".to_string());

    let mut result = String::new();

    for i in 0..symbols.getSize() {
        let sym = symbols.get(i);
        let val = dict.get(sym);

        while number >= val {
            result.push_str(sym);
            number -= val;
        }
    }

    println!("{}", result);
}