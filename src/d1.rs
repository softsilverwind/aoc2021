use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

pub fn simple()
{
    let ans: i32 = BufReader::new(io::stdin()).lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();
    println!("{}", ans);
}

pub fn complex()
{
    let ans: i32 = BufReader::new(io::stdin()).lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();
    println!("{}", ans);
}
