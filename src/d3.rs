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
    let diagnostic_report: Vec<Vec<u8>> =
        BufReader::new(io::stdin()).lines()
        .map(|line| line.unwrap())
        .map(|line|
            line.bytes()
            .filter_map(|byte|
                match byte {
                    b'0' => Some(0),
                    b'1' => Some(1),
                    _ => None
                }
            )
            .collect()
        )
        .collect();

    let mut oxygen = diagnostic_report.clone();
    let mut co2 = diagnostic_report;

    let mut i = 0;
    while oxygen.len() > 1 {
        let mut count = 0;       
        for entry in oxygen.iter() {
            if entry[i] == 1 {
                count += 1;
            }
            else {
                count -= 1;
            }
        }

        if count >= 0 {
            oxygen = oxygen.into_iter().filter(|entry| entry[i] == 1).collect();
        }
        else {
            oxygen = oxygen.into_iter().filter(|entry| entry[i] == 0).collect();
        }
        i += 1;
    }

    i = 0;
    while co2.len() > 1 {
        let mut count = 0;       
        for entry in co2.iter() {
            if entry[i] == 1 {
                count += 1;
            }
            else {
                count -= 1;
            }
        }

        if count < 0 {
            co2 = co2.into_iter().filter(|entry| entry[i] == 1).collect();
        }
        else {
            co2 = co2.into_iter().filter(|entry| entry[i] == 0).collect();
        }
        i += 1;
    }

    let oxygen_level = oxygen[0].iter().fold(0, |acc, elem|
        acc * 2 + if *elem > 0 { 1 } else { 0 }
    );

    let co2_level = co2[0].iter().fold(0, |acc, elem|
        acc * 2 + if *elem > 0 { 1 } else { 0 }
    );

    println!("{} * {} = {}", oxygen_level, co2_level, oxygen_level * co2_level);
}
