use std::io::{self, BufRead, BufReader};

pub fn simple()
{
    let mut lines = BufReader::new(io::stdin()).lines()
        .map(|line| line.unwrap());

    let mut acc: Vec<i32> = lines.next().unwrap()
        .bytes()
        .filter_map(|byte|
            match byte {
                b'0' => Some(-1),
                b'1' => Some(1),
                _ => None
            }
        )
        .collect();

    let ans = lines.fold(&mut acc, |acc, line| {
        let other = line
            .bytes()
            .filter_map(|byte|
                match byte {
                    b'0' => Some(-1),
                    b'1' => Some(1),
                    _ => None
                }
            );

        acc.iter_mut()
            .zip(other)
            .for_each(|(x, y)| *x += y);

        acc
    });

    let number = ans.iter().fold(0, |acc, elem|
        acc * 2 + if *elem > 0 { 1 } else { 0 }
    );

    let complement = ans.iter().fold(0, |acc, elem|
        acc * 2 + if *elem < 0 { 1 } else { 0 }
    );

    println!("{}", number * complement);
}

pub fn complex()
{
    unimplemented!()
}
