use std::{
    io::{self, BufRead, BufReader},
    ops::{Index, IndexMut},
};

#[derive(Default)]
struct Map
{
    elems: Vec<i8>,
    rows: i32,
    columns: i32
}

impl<T> FromIterator<T> for Map
    where T: AsRef<str>
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self
    {
        let mut elems = Vec::new();
        let mut count = 0;
        for line in iter {
            elems.extend(line
                .as_ref()
                .bytes()
                .filter(|&x| x >= b'0' && x <= b'9')
                .map(|x| (x - b'0') as i8)
            );
            count += 1;
        }
        let len = elems.len() as i32;

        Map {
            elems,
            rows: count,
            columns: len / count
        }
    }
}

impl Map
{
    pub fn rows(&self) -> i32 { self.rows }
    pub fn columns(&self) -> i32 { self.columns }

    pub fn new(rows: i32, columns: i32) -> Self
    {
        Map {
            elems: vec![0; (rows * columns) as usize],
            rows,
            columns
        }
    }
}

impl Index<(i32, i32)> for Map
{
    type Output = i8;

    fn index(&self, (i, j): (i32, i32)) -> &Self::Output
    {
        if i < 0 || i >= self.rows {
            &42
        }
        else if j < 0 || j >= self.columns {
            &42
        }
        else {
            &self.elems[(i * self.columns + j) as usize]
        }
    }
}

impl IndexMut<(i32, i32)> for Map
{
    fn index_mut(&mut self, (i, j): (i32, i32)) -> &mut Self::Output
    {
        &mut self.elems[(i * self.columns + j) as usize]
    }
}

pub fn simple()
{
    let map: Map = BufReader::new(io::stdin()).lines().map(|line| line.unwrap()).collect();

    let mut threat = 0;
    for i in 0..map.rows() {
        for j in 0..map.columns() {
            let min = *[
                map[(i-1, j  )],
                map[(i+1, j  )],
                map[(i  , j-1)],
                map[(i  , j+1)]
            ].iter().min().unwrap();

            if map[(i, j)] < min {
                threat += map[(i, j)] as i64 + 1;
            }
        }
    }
    println!("{}", threat);
}

fn dfs(map: &Map, marks: &mut Map, start_i: i32, start_j: i32) -> i32
{
    let mut front = vec![(start_i, start_j)];
    let mut count = 0;

    while let Some((i, j)) = front.pop() {
        if marks[(i, j)] == 0 && map[(i, j)] != 9 {
            count += 1;
            marks[(i, j)] = 1;
            front.push((i+1, j  ));
            front.push((i-1, j  ));
            front.push((i  , j+1));
            front.push((i  , j-1));
        }
    }

    count
}

pub fn complex()
{
    let map: Map = BufReader::new(io::stdin()).lines().map(|line| line.unwrap()).collect();
    let mut marks = Map::new(map.rows, map.columns);
    let mut sizes = Vec::new();

    for i in 0..map.rows() {
        for j in 0..map.columns() {
            if marks[(i, j)] == 0 && map[(i, j)] != 9 {
                sizes.push(dfs(&map, &mut marks, i, j));
            }
        }
    }
    sizes.sort_by_key(|x| -x);
    println!("{}", sizes[0] * sizes[1] * sizes[2]);    
}
