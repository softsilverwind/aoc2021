use std::{
    io::{self, BufReader, BufRead},
    collections::HashMap
};

struct Board
{
    numbers: Vec<Vec<Option<i32>>>,
    dispatch: HashMap<i32, (u8, u8)>
}

impl Board
{
    pub fn from_lines(lines: &mut impl Iterator<Item=String>) -> Self
    {
        let numbers: Vec<Vec<Option<i32>>> = (0..5)
            .map(|_| lines.next().unwrap())
            .map(|line|
                line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|numstr| Some(numstr.parse().unwrap()))
                .collect()
            )
            .collect();

        let mut dispatch = HashMap::new();
        for i in 0..5 {
            for j in 0..5 {
                dispatch.insert(numbers[i][j].unwrap(), (i as u8, j as u8));
            }
        }

        Board {
            numbers,
            dispatch
        }
    }

    pub fn maybe_remove(&mut self, val: i32)
    {
        if let Some(&(i, j)) = self.dispatch.get(&val) {
            self.numbers[i as usize][j as usize] = None;
        }
    }

    pub fn winner_points(&self) -> Option<i32>
    {
        let empty_line = self.numbers.iter().any(|line|
            line.iter().all(|elem| elem.is_none())
        );

        let mut empty_row = false;
        for j in 0..5 {
            if self.numbers.iter().map(|line| line[j]).all(|elem| elem.is_none()) {
                empty_row = true;
                break;
            }
        }

        if empty_line || empty_row {
            Some(self.numbers.iter().map(|line|
                line.iter().copied().filter_map(|x| x).sum::<i32>()
            ).sum())
        }
        else {
            None
        }
    }
}

pub fn simple()
{
    let mut input = BufReader::new(io::stdin()).lines().map(|line| line.unwrap());

    let numbers: Vec<i32> = input.next().unwrap().split(',').map(|numstr| numstr.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();

    while let Some(_) = input.next() {
        boards.push(Board::from_lines(&mut input))
    }

    for number in numbers {
        for board in boards.iter_mut() {
            board.maybe_remove(number);
            if let Some(score) = board.winner_points() {
                println!("{}", score * number);
                return;
            }
        }
    }
}

pub fn complex()
{
    let mut input = BufReader::new(io::stdin()).lines().map(|line| line.unwrap());

    let numbers: Vec<i32> = input.next().unwrap().split(',').map(|numstr| numstr.parse().unwrap()).collect();
    let mut boards: HashMap<usize, Board> = HashMap::new();

    let mut i = 0;
    while let Some(_) = input.next() {
        boards.insert(i, Board::from_lines(&mut input));
        i += 1;
    }

    let mut to_remove = Vec::new();

    for number in numbers {
        for (index, board) in boards.iter_mut() {
            board.maybe_remove(number);
            if let Some(_) = board.winner_points() {
                to_remove.push(*index);
            }
        }

        for index in to_remove.iter() {
            if boards.len() == 1 {
                println!("{}", boards[index].winner_points().unwrap() * number);
                return;
            }
            boards.remove(index);
        }
        to_remove.clear();
    }
}
