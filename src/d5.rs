use std::{
    io::{self, BufRead, BufReader},
    collections::HashMap
};

pub fn solve(simple: bool)
{
    let mut board: HashMap<(i32, i32), i32> = HashMap::new();

    for line in BufReader::new(io::stdin()).lines().map(|line| line.unwrap()) {
        let mut coords_iter = line
            .split(" -> ")
            .map(|coords|
                coords.split(',').map(|numstr| numstr.parse::<i32>().unwrap())
            )
            .flatten();

        let mut x1 = coords_iter.next().unwrap();
        let mut y1 = coords_iter.next().unwrap();
        let x2 = coords_iter.next().unwrap();
        let y2 = coords_iter.next().unwrap();

        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        if simple && dx != 0 && dy != 0 {
            continue;
        }

        while x1 != x2 || y1 != y2 {
            *board.entry((x1, y1)).or_insert(0) += 1;
            x1 += dx;
            y1 += dy;
        }
        *board.entry((x2, y2)).or_insert(0) += 1;
    }

    let ans = board.values().copied().filter(|&x| x > 1).count();
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
