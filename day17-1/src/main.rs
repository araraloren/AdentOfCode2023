use std::{iter::once, vec};

use neure::prelude::*;

const INPUT: &'static str = include_str!("../input.txt");

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

    // dbg!(find_minimum_path_2(
    //     (0, 0),
    //     (map.len() - 1, map[0].len() - 1),
    //     &map
    // ));
    Ok(())
}

pub fn find_minimum_path(start: (usize, usize), end: (usize, usize), map: &[Vec<usize>], rec: &mut Vec<Vec<usize>>) {
    let mut min = usize::MAX;
    let mut loss = 0;
    let mut path = vec![];
    let mut stack = vec![Point::new(start)];
    let mut forward = 0;

    while let Some(top) = stack.pop() {
        // if enter end or top is not have available direction
        if top.is_reach(end) || !top.avail() {
            if top.is_reach(end) { 
                if loss < min {
                    path = stack.iter().chain(once(&top)).map(|v|v.xy()).collect::<Vec<_>>();
                    min = loss;
                    println!("find a path {:?} with loss {}", &path, min);
                }
            }
            loss = loss.saturating_sub(map[top.x][top.y]);
            stack.last_mut().and_then(|v|Some(v.turn()));
        }
        // else if  {
            
        // }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: usize,

    y: usize,

    prev: Dir,

    dir: Dir,

    forward: usize,
}

impl Point {
    pub fn new((x, y): (usize, usize), prev: Dir) -> Self {
        Self {
            x,
            y,
            prev,
            dir: Dir::UP,
            forward: 0,
        }
    }

    pub fn avail(&self) -> bool {
        self.dir == Dir::END
    }

    pub fn is_reach(&self, point: (usize, usize)) -> bool {
        (self.x, self.y) == point
    }

    pub fn xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn turn(&mut self) {
        self.dir = Dir::new(self.dir as u8)
    }

    pub fn turn_back(&self) -> bool {
        match self.prev {
            Dir::UP => self.dir == Dir::DOWN,
            Dir::DOWN => self.dir == Dir::UP,
            Dir::LEFT => self.dir == Dir::RIGHT,
            Dir::RIGHT => self.dir == Dir::LEFT,
            Dir::END => true,
        }
    }

    pub fn forward_check(&self) -> bool {
        self.forward == 3 && self.prev == self.dir
    }

    // prev -?> point -?> next
    pub fn next(&mut self, start: (usize, usize), end: (usize, usize)) -> Option<(usize, usize)> {
        while self.avail() && (self.turn_back() || self.forward_check()) {
            self.turn();
        }
        if self.avail() {
            todo!()
        }
        else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    END,
}

impl Dir {
    pub fn new(val: u8) -> Self {
        match val {
            0 => Self::UP,
            1 => Self::DOWN,
            2 => Self::LEFT,
            3 => Self::RIGHT,
            4 => Self::END,
            _ => {
                unreachable!("")
            }
        }
    }

    pub fn next(&self, loc: (usize, usize), start: (usize, usize), end: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = loc;

        match self {
            Dir::UP => {
                if x == start.0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Dir::DOWN => {
                if x == end.0 - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Dir::LEFT => {
                if y == start.1 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Dir::RIGHT => {
                if y == end.1 - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Dir::END => None,
        }
    }
}
