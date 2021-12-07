use std::io::{self, BufRead, BufReader};

fn solve(simple: bool)
{
    let mut lanternfish = [0; 7];
    let mut buffer = [0; 2];

    for x in BufReader::new(io::stdin()).lines().next().unwrap().unwrap().split(',') {
        lanternfish[x.parse::<usize>().unwrap()] += 1;
    }

    let mut i = 0;
    let mut bi = 0;
    let limit = if simple { 80 } else { 256 };
    for _gen in 0..limit {
        let buf = buffer[bi];
        buffer[bi] = lanternfish[i];
        lanternfish[i] += buf;

        i = (i + 1) % 7;
        bi = (bi + 1) % 2;
    }

    let ans = lanternfish.iter().sum::<i64>() + buffer.iter().sum::<i64>();
    println!("{}", ans);
}

pub fn simple()
{
    solve(true);
}

pub fn complex()
{
    solve(false);
}
