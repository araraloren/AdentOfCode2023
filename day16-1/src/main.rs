use std::vec;

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");
const LOG: &str = include_str!("../x.log");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let es = ".".map(|_| Ok(Tile::ES));
    let lm = "/".map(|_| Ok(Tile::LM));
    let rm = "\\".map(|_| Ok(Tile::RM));
    let hs = "-".map(|_| Ok(Tile::HS));
    let vs = "|".map(|_| Ok(Tile::VS));
    let re = es
        .or(lm)
        .or(rm)
        .or(hs)
        .or(vs)
        .collect::<_, Vec<_>>()
        .at_least(1)
        .sep("\n")
        .quote(re::start(), re::end());
    let mut grid = RegexCtx::new(INPUT).ctor(&re)?;
    let mut lights = vec![((22, 109), Dir::L)];
    let (row, col) = (grid.len(), grid[0].len());
    let mut flags = vec![vec![vec![]; col]; row];

    while !lights.is_empty() {
        let mut next = vec![];

        for light in lights {
            let ((x, y), dir) = light;
            let tile = &mut grid[x][y];
            let flag = &mut flags[x][y];

            if !flag.contains(&dir) {
                flag.push(dir);
            }
            for n_dir in tile.moving(dir) {
                if !n_dir.end(x, y, row, col) {
                    let (nx, ny) = n_dir.next(x, y);

                    if !flags[nx][ny].contains(&n_dir) {
                        next.push(((nx, ny), n_dir));
                    }
                }
            }
        }
        lights = next;
    }

    let num = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str::<usize>());
    let log_re = num
        .sep_once(",", num)
        .quote("(", ")")
        .sep(",")
        .quote("{", "}");

    let values = CharsCtx::new(LOG)
        .ignore(neu::whitespace().repeat_full())
        .ctor(&log_re)?;

    dbg!(&values);

    for (x, line) in grid.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            if !flags[x][y].is_empty() {
                //print!("({}, {})", x, y);
                if !values.contains(&(x, y)) {
                    println!("----------- got {} {} ", x, y);
                }
            }
        }
        println!();
    }
    let mut count = 0;

    for (x, line) in grid.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            if flags[x][y].is_empty() {
                print!(".");
            } else {
                count += 1;
                print!("#");
            }
        }
        println!();
    }
    println!("tiles that are energized: {}", count);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn next(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Dir::U => (i - 1, j),
            Dir::D => (i + 1, j),
            Dir::L => (i, j - 1),
            Dir::R => (i, j + 1),
        }
    }

    pub fn end(&self, i: usize, j: usize, row: usize, col: usize) -> bool {
        match self {
            Dir::U => i == 0,
            Dir::D => i == row - 1,
            Dir::L => j == 0,
            Dir::R => j == col - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    /// empty space '.'
    ES,
    /// left mirrors '/'
    LM,
    /// right mirrors '\'
    RM,
    /// horizontal splitter '-'
    HS,
    /// veritcal splitter '|'
    VS,
}

impl Tile {
    pub fn moving(&self, dir: Dir) -> Vec<Dir> {
        let mut ret = vec![];

        match self {
            Tile::ES => ret.push(dir),
            Tile::LM => {
                ret.push(match dir {
                    Dir::U => Dir::R,
                    Dir::D => Dir::L,
                    Dir::L => Dir::D,
                    Dir::R => Dir::U,
                });
            }
            Tile::RM => {
                ret.push(match dir {
                    Dir::U => Dir::L,
                    Dir::D => Dir::R,
                    Dir::L => Dir::U,
                    Dir::R => Dir::D,
                });
            }
            Tile::HS => match dir {
                Dir::U | Dir::D => {
                    ret.push(Dir::L);
                    ret.push(Dir::R);
                }
                Dir::L => {
                    ret.push(Dir::L);
                }
                Dir::R => {
                    ret.push(Dir::R);
                }
            },
            Tile::VS => match dir {
                Dir::L | Dir::R => {
                    ret.push(Dir::U);
                    ret.push(Dir::D);
                }
                Dir::U => {
                    ret.push(Dir::U);
                }
                Dir::D => {
                    ret.push(Dir::D);
                }
            },
        }
        ret
    }
}
