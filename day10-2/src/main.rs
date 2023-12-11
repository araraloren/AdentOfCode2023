use neure::{err, prelude::*};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipe = neu!(('|', '-', 'L', 'J', '7', 'F', '.', 'S'))
        .repeat_one()
        .map(Kind::new)
        .repeat(1..)
        .sep("\n");
    let kinds = RegexCtx::new(INPUT).ctor(&pipe)?;
    let pipes = kinds
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, kind)| Pipe::new(*kind, i, j, &kinds))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("--> {} {}", pipes.len(), pipes[0].len());

    let mut stack = vec![];
    let mut inside = vec![];

    for line in pipes.iter() {
        for pipe in line.iter() {
            if matches!(pipe.kind, Kind::ST) {
                stack.push(pipe.clone());
                break;
            }
        }
    }
    println!("using start position ({:?})", stack.last().unwrap());
    while !stack.is_empty() {
        if let Some(top) = stack.last_mut() {
            if let Some((x, y)) = top.next_p() {
                let mut next = pipes.get(x).and_then(|v| v.get(y)).unwrap().clone();

                next.remove(top.coord);
                if stack.first().unwrap().coord == next.coord {
                    break;
                }
                stack.push(next);
            } else {
                stack.pop();
            }
        }
    }
    for line in pipes.iter() {
        for pipe in line.iter() {
            if !stack.iter().any(|v| v.coord == pipe.coord) {
                let (x, y) = pipe.coord;
                let mut count = 0;
                let mut last = None;
                let mut cross = stack
                    .iter()
                    .filter(|v| v.coord.0 == x && v.coord.1 > y)
                    .collect::<Vec<_>>();

                // sort the cross by y
                cross.sort_by(|a, b| a.coord.1.partial_cmp(&b.coord.1).unwrap());
                for pipe in &cross {
                    match pipe.kind {
                        Kind::NE | Kind::SE => {
                            assert!(last.is_none());
                            last = Some(pipe);
                        }
                        Kind::NW => {
                            assert!(last.is_some());
                            if !matches!(last.unwrap().kind, Kind::NE) {
                                count += 1;
                            }
                            last = None;
                        }
                        Kind::SW => {
                            assert!(last.is_some());
                            if !matches!(last.unwrap().kind, Kind::SE) {
                                count += 1;
                            }
                            last = None;
                        }
                        Kind::NS | Kind::ST => {
                            count += 1;
                            last = None;
                        }
                        _ => {}
                    }
                }
                if count % 2 == 1 {
                    inside.push(pipe.clone());
                }
            }
        }
    }

    println!("--> {:?} len = {}", inside, inside.len());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    GN,
    EW,
    NE,
    NW,
    SE,
    SW,
    NS,
    ST,
}

impl Kind {
    pub fn new(val: &str) -> Result<Self, err::Error> {
        Ok(match val {
            "|" => Self::NS,
            "-" => Self::EW,
            "L" => Self::NE,
            "J" => Self::NW,
            "7" => Self::SW,
            "F" => Self::SE,
            "S" => Self::ST,
            _ => {
                assert_eq!(val, ".");
                Self::GN
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Pipe {
    kind: Kind,

    coord: (usize, usize),

    index: usize,

    paths: Vec<(usize, usize)>,
}

impl Pipe {
    pub fn new(kind: Kind, i: usize, j: usize, kinds: &[Vec<Kind>]) -> Self {
        let dirs = [
            vec![],
            vec![Dir::L, Dir::R],
            vec![Dir::U, Dir::R],
            vec![Dir::U, Dir::L],
            vec![Dir::D, Dir::R],
            vec![Dir::D, Dir::L],
            vec![Dir::U, Dir::D],
            vec![Dir::U, Dir::D, Dir::L, Dir::R],
        ];
        let mut paths = vec![];

        for path in &dirs[kind as usize] {
            if let Some(next) = path.next((i, j), kinds) {
                paths.push(next);
            }
        }
        Self {
            kind,
            paths,
            index: 0,
            coord: (i, j),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.paths.len()
    }

    pub fn next_p(&mut self) -> Option<(usize, usize)> {
        let ret = self.paths.get(self.index).copied();
        self.index += 1;
        ret
    }

    pub fn remove(&mut self, coord: (usize, usize)) {
        if let Some(idx) = self.paths.iter().position(|v| *v == coord) {
            self.paths.remove(idx);
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
    pub fn next(&self, (i, j): (usize, usize), kinds: &[Vec<Kind>]) -> Option<(usize, usize)> {
        match self {
            Dir::U => {
                if i > 0 {
                    if let Some(kind) = kinds.get(i - 1).and_then(|v| v.get(j)) {
                        if matches!(kind, Kind::SW | Kind::SE | Kind::NS | Kind::ST) {
                            return Some((i - 1, j));
                        }
                    }
                }
            }
            Dir::D => {
                if let Some(kind) = kinds.get(i + 1).and_then(|v| v.get(j)) {
                    if matches!(kind, Kind::NW | Kind::NE | Kind::NS | Kind::ST) {
                        return Some((i + 1, j));
                    }
                }
            }
            Dir::L => {
                if j > 0 {
                    if let Some(kind) = kinds.get(i).and_then(|v| v.get(j - 1)) {
                        if matches!(kind, Kind::NE | Kind::SE | Kind::EW | Kind::ST) {
                            return Some((i, j - 1));
                        }
                    }
                }
            }
            Dir::R => {
                if let Some(kind) = kinds.get(i).and_then(|v| v.get(j + 1)) {
                    if matches!(kind, Kind::NW | Kind::SW | Kind::EW | Kind::ST) {
                        return Some((i, j + 1));
                    }
                }
            }
        }
        None
    }
}
