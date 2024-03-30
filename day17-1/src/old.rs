use std::{iter::once, vec};

use neure::prelude::*;

const INPUT: &'static str = include_str!("../input.txt");

pub type Loc = (usize, usize);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = neu::digit(10)
        .repeat_one()
        .map(re::map::from_str::<usize>())
        .collect::<_, Vec<_>>()
        .at_least(1)
        .sep("\n");
    let map = CharsCtx::new(INPUT).ctor(&re)?;

    for len in (0 .. map.len() - 1).rev() {
        if let Some(part) = map.get(len .. map.len()) {
            
        }
    }

    dbg!(find_minimum_path_2(
        (0, 0),
        (map.len() - 1, map[0].len() - 1),
        &map
    ));
    Ok(())
}

pub fn find_minimum_path(start: Loc, end: Loc, map: &[Vec<usize>], rec: &mut Vec<Vec<usize>>) {
    let mut min = usize::MAX;
    let mut loss = 0;
    let mut path = vec![];
    let mut stack = vec![(start, [true, true, true, true])];
    let mut forward = 0;

    while let Some(top) = stack.pop() {
        let ((x, y), dirs) = top;

        // if enter end or top is not have available direction
        if (x, y) == end || dirs.iter().all(|v|*v) {
            if (x, y) == end {
                if loss < min {
                    path = stack.iter().map(|v|v.0).chain(once((x, y))).collect::<Vec<_>>();
                    min = loss;
                    println!("find a path {:?} with loss {}", &path, min);
                }
            }
            loss = loss.saturating_sub(map[x][y]);
            if let Some(last) = stack.last_mut(){
                if let Some(dir) = last.1.iter_mut().find(|v| **v) {
                    *dir = false;
                }
            }
        }
        else if  {
            
        }
    }
}

pub fn find_minimum_path_2(start: Loc, end: Loc, map: &[Vec<usize>]) -> Vec<Loc> {
    let (row, col) = (map.len(), map[0].len());
    let mut path = vec![];
    let mut min = usize::MAX;
    let mut loss = 0;
    let mut stack = vec![(start, Dir::U)];

    while !stack.is_empty() {
        if let Some(top) = stack.pop() {
            let ((x, y), dir) = top;

            if (x, y) == end || matches!(dir, Dir::O) {
                if (x, y) == end {
                    if loss < min {
                        path = stack
                            .iter()
                            .map(|v| v.0)
                            .chain(once((x, y)))
                            .collect::<Vec<_>>();
                        min = loss;
                        println!("find a path {:?} with loss {}", &path, min);
                    }
                }
                loss = loss.saturating_sub(map[x][y]);
                stack
                    .last_mut()
                    .iter_mut()
                    .for_each(|(_, dir)| *dir = dir.next_dir());
            } else if let Some((nx, ny)) = dir.next_2((x, y), row, col) {
                let has_loc = stack.iter().any(|((x, y), _)| (x, y) == (&nx, &ny));

                if !has_loc {
                    loss += map[nx][ny];
                    stack.push(((x, y), dir));
                    stack.push(((nx, ny), Dir::U));
                } else {
                    stack.push(((x, y), dir.next_dir()));
                }
            } else {
                stack.push(((x, y), dir.next_dir()));
            }
        }
    }
    println!("find a path {:?} with loss {}", &path, min);
    path
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    U,
    D,
    L,
    R,
    O,
}

impl Dir {
    pub fn new(val: usize) -> Self {
        match val {
            0 => Self::U,
            1 => Self::D,
            2 => Self::L,
            3 => Self::R,
            4 => Self::O,
            _ => {
                unreachable!("")
            }
        }
    }

    pub fn next_dir(&self) -> Self {
        match self {
            Dir::U => Self::D,
            Dir::D => Self::L,
            Dir::L => Self::R,
            Dir::R => Self::O,
            Dir::O => Self::O,
        }
    }

    pub fn next_2(&self, (i, j): Loc, row: usize, col: usize) -> Option<Loc> {
        match self {
            Dir::U => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            Dir::D => {
                if i == row - 1 {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            Dir::L => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
            Dir::R => {
                if j == col - 1 {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            Dir::O => None,
        }
    }

    pub fn next(&self, loc: Loc, start: Loc, end: Loc, forward: usize) -> Option<Loc> {

    }
}
