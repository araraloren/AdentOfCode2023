use std::{collections::HashSet, vec};

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

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
    let grid = RegexCtx::new(INPUT).ctor(&re)?;
    let (row, col) = (grid.len(), grid[0].len());
    let mut es = vec![vec![vec![None; 4]; col]; row];

    for (i, line) in grid.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            for d in [Dir::U, Dir::D, Dir::L, Dir::R] {
                let mut loc = vec![vec![vec![None; 4]; col]; row];
                let mut ls = vec![Lighting::new(((i, j), d))];
                let mut stop = false;

                while !stop {
                    let mut new_ls = ls.clone();
                    let old_len = ls.len();

                    for (idx, l) in ls.iter_mut().enumerate() {
                        l.go(idx, &grid, &es, &mut new_ls, &mut loc);
                    }
                    for (idx, l) in new_ls.into_iter().enumerate() {
                        if idx >= old_len {
                            ls.push(l);
                        }
                    }
                    stop = ls.iter().all(|v| v.stop);
                }
                for lp in ls.iter().rev() {
                    lp.collect_to(&mut es, &ls, row, col, &loc);
                }
                println!("process at ({}, {}, {:?})", i, j, d);
            }
        }
    }
    let mut max = 0;
    let mut start = ((0, 0), Dir::U);

    for (x, line) in es.iter().enumerate() {
        for (y, col) in line.iter().enumerate() {
            for (z, set) in col.iter().enumerate() {
                if set.is_some() {
                    let len = set.as_ref().unwrap().len();

                    if len > max {
                        max = len;
                        start = ((x, y), Dir::new(z));
                    }
                }
            }
        }
    }

    println!(
        "--> you can get {} energized when start at {:?}",
        max, start
    );

    Ok(())
}

pub type EnergizedSet = Vec<Vec<Vec<Option<HashSet<(usize, usize)>>>>>;
pub type LocationVec = Vec<Vec<Vec<Option<(usize, usize)>>>>;

#[derive(Debug, Clone)]
pub struct Lighting {
    path: Vec<((usize, usize), Dir)>,

    stop: bool,

    found: bool,

    child: Vec<((usize, usize), Dir)>,
}

impl Lighting {
    pub fn new(start: ((usize, usize), Dir)) -> Self {
        Self {
            path: vec![start],
            stop: false,
            found: false,
            child: vec![],
        }
    }

    pub fn is_start(&self, start: &((usize, usize), Dir)) -> bool {
        self.path.first().unwrap() == start
    }

    pub fn go(
        &mut self,
        li: usize,
        tiles: &[Vec<Tile>],
        es: &EnergizedSet,
        ls: &mut Vec<Lighting>,
        loc: &mut LocationVec,
    ) {
        if !self.stop {
            let (row, col) = (tiles.len(), tiles[0].len());
            let ((x, y), dir) = *self.path.last().unwrap();
            let idx = dir as usize;

            if es[x][y][idx].is_none() {
                let tile = &tiles[x][y];
                let n_dirs = tile.moving(dir);
                let n_len = n_dirs.len();

                loc[x][y][idx] = Some((li, self.path.len() - 1));
                if n_len == 1 {
                    let n_dir = n_dirs[0];

                    if n_dir.end(x, y, row, col) {
                        self.stop = true;
                    } else {
                        let ((nx, ny), nz) = (n_dir.next(x, y), n_dir as usize);

                        if loc[nx][ny][nz].is_some() {
                            self.stop = true;
                            self.child.push(((nx, ny), n_dir));
                        } else {
                            self.path.push(((nx, ny), n_dir));
                        }
                    }
                } else {
                    self.stop = true;
                    for n_dir in n_dirs {
                        if !n_dir.end(x, y, row, col) {
                            let ((nx, ny), nz) = (n_dir.next(x, y), n_dir as usize);

                            self.child.push(((nx, ny), n_dir));
                            if loc[nx][ny][nz].is_none() {
                                ls.push(Lighting::new(((nx, ny), n_dir)));
                            }
                        }
                    }
                }
            } else {
                if loc[x][y][idx].is_none() {
                    loc[x][y][idx] = Some((li, self.path.len() - 1));
                }
                self.found = true;
                self.stop = true;
            }
        }
    }

    pub fn collect_lp(
        &self,
        es: &EnergizedSet,
        ls: &Vec<Lighting>,
        flags: &mut Vec<Vec<Vec<bool>>>,
        pos: usize,
        rec: &LocationVec,
        ret: &mut HashSet<(usize, usize)>,
    ) {
        if self.path.get(pos).is_some() {
            for path in self.path.iter().skip(pos) {
                let ((x, y), dir) = *path;
                let z = dir as usize;

                if let Some(set) = &es[x][y][z] {
                    ret.extend(set.clone());
                    return;
                } else if !flags[x][y][z] {
                    flags[x][y][z] = true;
                    ret.insert(path.0);
                }
            }
            for child in self.child.iter() {
                let ((x, y), dir) = *child;
                let z = dir as usize;

                if !flags[x][y][z] {
                    let index = rec[x][y][z].unwrap();

                    ls[index.0].collect_lp(es, ls, flags, index.1, rec, ret);
                }
            }
        }
    }

    pub fn collect_to(
        &self,
        es: &mut EnergizedSet,
        ls: &Vec<Lighting>,
        row: usize,
        col: usize,
        rec: &LocationVec,
    ) {
        for (pos, path) in self.path.iter().enumerate().rev() {
            let mut flags = vec![vec![vec![false; 4]; col]; row];
            let ((x, y), dir) = *path;

            if es[x][y][dir as usize].is_none() {
                let mut set = HashSet::default();

                self.collect_lp(es, ls, &mut flags, pos, rec, &mut set);
                es[x][y][dir as usize] = Some(set);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn new(val: usize) -> Self {
        match val {
            0 => Self::U,
            1 => Self::D,
            2 => Self::L,
            3 => Self::R,
            _ => panic!("............."),
        }
    }

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
