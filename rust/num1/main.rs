#[path = "../structs/mod.rs"]
mod structs;

use crate::structs::stack::Stack;

use std::io;

fn solve_top(nums: &mut Stack<i64>, opers: &mut Stack<char>) -> Result<(), String> {
    if opers.isEmpty() || *opers.front() == '(' {
        return Err("expected operator".to_string());
    }

    if nums.isEmpty() {
        return Err("not enough operands".to_string());
    }

    let b = nums.pop();

    if nums.isEmpty() {
        return Err("not enough operands".to_string());
    }

    let a = nums.pop();

    let op = opers.pop();
    let res = match op {
        '*' => a * b,
        '/' => {
            if b == 0 {
                return Err("division by zero".to_string());
            }
            a / b
        }
        '+' => a + b,
        '-' => a - b,
        _ => return Err("unknown operator".to_string()),
    };

    nums.push(res);
    Ok(())
}

fn pop_num_buff(s: &mut String, out: &mut i64) -> Result<bool, String> {
    if s.is_empty() {
        return Ok(false);
    }

    if s == "-" {
        return Err("dangling unary minus".to_string());
    }

    match s.parse::<i64>() {
        Ok(v) => {
            *out = v;
            s.clear();
            Ok(true)
        }
        Err(_) => Err(format!("invalid number: '{}'", s)),
    }
}

fn solve(input: &str) -> Result<i64, String> {
    let mut nums: Stack<i64> = Stack::new();
    let mut opers: Stack<char> = Stack::new();

    let mut str_num = String::new();
    let mut cur_num: i64 = 0;
    let mut next_unary = true;

    for ch in input.chars() {
        match ch {
            ' ' => {}

            '*' | '/' => {
                if pop_num_buff(&mut str_num, &mut cur_num)? {
                    nums.push(cur_num);
                }
                if !opers.isEmpty() && *opers.front() != '(' &&
                    (*opers.front() == '*' || *opers.front() == '/') {
                    solve_top(&mut nums, &mut opers)?;
                }
                opers.push(ch);
                next_unary = true;
            }

            '+' | '-' => {
                if ch == '-' && next_unary {
                    str_num.push('-');
                    continue;
                }
                if pop_num_buff(&mut str_num, &mut cur_num)? {
                    nums.push(cur_num);
                }
                if !opers.isEmpty() && *opers.front() != '(' {
                    solve_top(&mut nums, &mut opers)?;
                }
                opers.push(ch);
                next_unary = true;
            }

            '(' => {
                if str_num == "-" {
                    nums.push(-1);
                    opers.push('*');
                    str_num.clear();
                }
                opers.push(ch);
                next_unary = true;
            }

            ')' => {
                if pop_num_buff(&mut str_num, &mut cur_num)? {
                    nums.push(cur_num);
                }
                while !opers.isEmpty() && *opers.front() != '(' {
                    solve_top(&mut nums, &mut opers)?;
                }
                if opers.isEmpty() {
                    return Err("unmatched ')'".to_string());
                }
                opers.pop();
                next_unary = false;
            }

            _ => {
                str_num.push(ch);
                next_unary = false;
            }
        }
    }

    if pop_num_buff(&mut str_num, &mut cur_num)? {
        nums.push(cur_num);
    }

    while !opers.isEmpty() {
        if *opers.front() == '(' {
            return Err("unmatched '('".to_string());
        }
        solve_top(&mut nums, &mut opers)?;
    }

    if nums.isEmpty() {
        return Err("empty expression".to_string());
    }

    let result = nums.pop();

    if !nums.isEmpty() {
        return Err("trailing operands in expression".to_string());
    }

    Ok(result)
}

fn main() {
    let mut line = String::new();

    if io::stdin().read_line(&mut line).is_err() {
        eprintln!("error: failed to read input");
        std::process::exit(1);
    }

    let line = line.trim_end_matches(&['\n', '\r'][..]);

    match solve(line) {
        Ok(v) => println!("solve: {}", v),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}