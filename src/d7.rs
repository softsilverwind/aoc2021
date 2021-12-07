use std::io::{self, BufRead, BufReader};

fn solve(simple: bool)
{
    let input: Vec<usize> = BufReader::new(io::stdin()).lines().next().unwrap().unwrap()
        .split(',')
        .map(|numstr| numstr.parse::<usize>().unwrap())
        .collect();

    let length = *input.iter().max().unwrap() + 1;

    let mut arena = vec![0; length];

    for i in input.into_iter() {
        arena[i] += 1;
    }

    let mut best_i = 0;
    let mut best = i64::MAX;

    // Not proud of the complexity of that, oh well
    for base_i in 0..length {
        let mut count = 0;
        for (i, elem) in arena.iter().enumerate() {
            if simple {
                count += elem * (base_i as i64 - i as i64).abs();
            }
            else {
                let delta = (base_i as i64 - i as i64).abs();
                count += elem * (delta + 1) * delta / 2;
            }
        }

        if count < best {
            best = count;
            best_i = base_i;
        }
        else {
            break;
        }
    }

    println!("{}: {}", best_i, best);
}

pub fn simple()
{
    solve(true);
}

pub fn complex()
{
    solve(false);
}
