#[path = "../structs/mod.rs"]
mod structs;

use crate::structs::array_list::ArrayList;
use std::env;

fn match_pattern(s: &ArrayList<char>, r: &ArrayList<char>) -> bool {
    let mut si = 0;
    let mut ri = 0;
    let mut has_star = false;
    let mut star_pos = 0;
    let mut star_match = 0;

    while si < s.getSize() {
        if ri < r.getSize() && (*r.get(ri) == '?' || *r.get(ri) == *s.get(si)) {
            si += 1;
            ri += 1;
        } else if ri < r.getSize() && *r.get(ri) == '*' {
            has_star = true;
            star_pos = ri;
            ri += 1;
            star_match = si;
        } else if has_star {
            ri = star_pos + 1;
            star_match += 1;
            si = star_match;
        } else {
            return false;
        }
    }

    while ri < r.getSize() && *r.get(ri) == '*' {
        ri += 1;
    }

    ri == r.getSize()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: ./main <string> <pattern>");
        std::process::exit(1);
    }

    let s_str = &args[1];
    let r_str = &args[2];

    let mut s = ArrayList::new();
    let mut r = ArrayList::new();

    for c in s_str.chars() {
        s.pushBack(c);
    }
    for c in r_str.chars() {
        r.pushBack(c);
    }

    if match_pattern(&s, &r) {
        println!("TRUE");
    } else {
        println!("FALSE");
    }
}