use std::path::PathBuf;
use std::ptr::addr_of;
use std::str::Chars;

use crate::util::read_input;

static NUMBERS: &'static [&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_char_digit<C: Iterator<Item = (usize, char)>>(seq: C) -> Option<(usize, usize)> {
    seq.filter(|(_, it)| it.is_numeric())
        .next()
        .map(|(i, it)| (i, (it as u8 - '0' as u8) as usize))
}

pub fn main() {
    let input = read_input(1);
    let lines = input.split('\n').filter(|it| !it.is_empty());
    let mut sum: usize = 0;
    for l in lines {
        let (mut fi, mut first) = get_char_digit(l.chars().enumerate()).unwrap();
        for (value, nstr) in NUMBERS.iter().enumerate().skip(1) {
            match l.find(nstr) {
                Some(at) if at < fi => {
                    fi = at;
                    first = value;
                }
                _ => {}
            }
        }
        let (mut li, mut last) = get_char_digit(l.chars().rev().enumerate())
            .map(|(i, it)| (l.len() - 1 - i, it))
            .unwrap();
        for (value, nstr) in NUMBERS.iter().enumerate().skip(1) {
            match l.rfind(nstr) {
                Some(at) if at + nstr.len() > li => {
                    li = at + nstr.len();
                    last = value;
                }
                _ => {}
            }
        }

        sum += first * 10 + last;
    }
    println!("{}", sum);
}
