use std::io::{self, BufRead, BufReader};

pub fn simple()
{
    let (x, y) = BufReader::new(io::stdin()).lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut iter = line.split(' ');
            let cmd = iter.next().unwrap();
            let amount: i32 = iter.next().unwrap().parse().unwrap();

            match cmd {
                "forward" => (amount, 0),
                "down" => (0, amount),
                "up" => (0, -amount),
                _ => panic!("Invalid input file")
            }
        })
        .fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    println!("{}", x * y);
}

pub fn complex()
{
    let (x, y, _) = BufReader::new(io::stdin()).lines()
        .map(|line| line.unwrap())
        .fold((0, 0, 0), |(x, y, aim), line| {
            let mut iter = line.split(' ');
            let cmd = iter.next().unwrap();
            let amount: i64 = iter.next().unwrap().parse().unwrap();

            match cmd {
                "forward" => (x + amount, y + aim * amount, aim),
                "down" => (x, y, aim + amount),
                "up" => (x, y, aim - amount),
                _ => panic!("Invalid input file")
            }
        });

    println!("{}", x * y);
}
