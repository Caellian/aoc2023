use std::path::PathBuf;
use std::ptr::addr_of;

use crate::util::read_input;

pub fn main() {
    let input = read_input(1);
    let lines = input.split('\n');
    let mut sum: usize = 0;
    for l in lines {
        let first = l.chars().filter(|it| it.is_numeric()).next().unwrap();
        let last = l.chars().rev().filter(|it| it.is_numeric()).next().unwrap();
        sum += format!("{}{}", first, last).parse::<usize>().unwrap()
    }
    println!("{}", sum);
}
