use std::io::{self, BufRead, BufReader};

fn solve(simple: bool)
{
    let mut illegal = Vec::new();
    let mut missing = Vec::new();

    for line in BufReader::new(io::stdin()).lines().map(|line| line.unwrap()) {
        let mut stack: Vec<u8> = Vec::new();
        for byte in line.bytes().filter(|x| x.is_ascii_punctuation()) {
            match byte {
                b'(' => stack.push(byte + 1),
                b'[' | b'{' | b'<' => stack.push(byte + 2),
                b')' | b']' | b'}' | b'>' => {
                    if stack.pop().unwrap_or(b'*') != byte {
                        illegal.push(byte);
                        stack.clear();
                        break;
                    }
                },
                _ => panic!("Invalid input")
            }
        }
        if !stack.is_empty() {
            missing.push(stack)
        }
    }


    if simple {
        let mut points: i64 = 0;
        for c in illegal {
            points += match c {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                b'>' => 25137,
                _ => panic!("My brain just exploded")
            }
        }
        println!("{}", points);
    }
    else {
        let mut points = Vec::new();
        for line in missing.iter_mut() {
            let mut line_points: i64 = 0;
            while let Some(c) = line.pop() {
                line_points *= 5;
                line_points += match c {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => panic!("My brain just exploded")
                };
            }
            points.push(line_points);
        }

        // The optimal way is to use quickselect, but let's not add a crate dependency just for that...
        points.sort();
        println!("{}", points[points.len() / 2]);
    }
}

pub fn simple()
{
    solve(true)
}

pub fn complex()
{
    solve(false)
}
