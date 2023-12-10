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
    let mut all_stack = vec![];

    for line in pipes.iter() {
        for pipe in line.iter() {
            if matches!(pipe.kind, Kind::ST) {
                stack.push((pipe.clone(), 0));
                break;
            }
        }
    }
    println!("using start position ({:?})", stack.last().unwrap());
    while !stack.is_empty() {
        if let Some((top, offset)) = stack.last_mut() {
            if *offset >= top.paths.len() {
                stack.pop();
            } else {
                let (x, y) = top.paths[*offset];
                let mut next_pipe = pipes.get(x).and_then(|v| v.get(y)).unwrap().clone();

                *offset += 1;
                if let Some(idx) = next_pipe.paths.iter().position(|v| *v == top.coord) {
                    next_pipe.paths.remove(idx);
                }
                let is_cycle = stack.first().unwrap().0.coord == (x, y);

                stack.push((next_pipe, 0));
                if is_cycle {
                    all_stack.push(stack.clone());
                    stack.pop();
                }
            }
        }
    }

    for stack in all_stack {
        println!(
            "start is {:?}, end is {:?}, length = {}, farthest = {}",
            stack[0],
            stack[stack.len() - 2],
            stack.len(),
            (stack.len() - 1) / 2,
        );
    }

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

#[derive(Debug, Clone)]
pub struct Pipe {
    kind: Kind,

    coord: (usize, usize),

    paths: Vec<(usize, usize)>,
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

impl Pipe {
    pub fn new(kind: Kind, i: usize, j: usize, kinds: &[Vec<Kind>]) -> Self {
        let mut paths = vec![];

        match kind {
            Kind::GN => {}
            Kind::ST => {
                if i > 0 {
                    if let Some(pipe) = kinds.get(i - 1).and_then(|v| v.get(j)) {
                        if matches!(pipe, Kind::SW | Kind::SE | Kind::NS) {
                            paths.push((i - 1, j));
                        }
                    }
                }
                if let Some(pipe) = kinds.get(i + 1).and_then(|v| v.get(j)) {
                    if matches!(pipe, Kind::NW | Kind::NE | Kind::NS) {
                        paths.push((i + 1, j));
                    }
                }
                if j > 0 {
                    if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j - 1)) {
                        if matches!(pipe, Kind::NE | Kind::SE | Kind::EW) {
                            paths.push((i, j - 1));
                        }
                    }
                }
                if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j + 1)) {
                    if matches!(pipe, Kind::NW | Kind::SW | Kind::EW) {
                        paths.push((i, j + 1));
                    }
                }
            }
            Kind::EW => {
                if j > 0 {
                    if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j - 1)) {
                        if matches!(pipe, Kind::NE | Kind::SE | Kind::EW | Kind::ST) {
                            paths.push((i, j - 1));
                        }
                    }
                }
                if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j + 1)) {
                    if matches!(pipe, Kind::NW | Kind::SW | Kind::EW | Kind::ST) {
                        paths.push((i, j + 1));
                    }
                }
            }
            Kind::NE => {
                if i > 0 {
                    if let Some(pipe) = kinds.get(i - 1).and_then(|v| v.get(j)) {
                        if matches!(pipe, Kind::SW | Kind::SE | Kind::NS | Kind::ST) {
                            paths.push((i - 1, j));
                        }
                    }
                }
                if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j + 1)) {
                    if matches!(pipe, Kind::NW | Kind::SW | Kind::EW | Kind::ST) {
                        paths.push((i, j + 1));
                    }
                }
            }
            Kind::NW => {
                if i > 0 {
                    if let Some(pipe) = kinds.get(i - 1).and_then(|v| v.get(j)) {
                        if matches!(pipe, Kind::SW | Kind::SE | Kind::NS | Kind::ST) {
                            paths.push((i - 1, j));
                        }
                    }
                }
                if j > 0 {
                    if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j - 1)) {
                        if matches!(pipe, Kind::NE | Kind::SE | Kind::EW | Kind::ST) {
                            paths.push((i, j - 1));
                        }
                    }
                }
            }
            Kind::SE => {
                if let Some(pipe) = kinds.get(i + 1).and_then(|v| v.get(j)) {
                    if matches!(pipe, Kind::NW | Kind::NE | Kind::NS | Kind::ST) {
                        paths.push((i + 1, j));
                    }
                }
                if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j + 1)) {
                    if matches!(pipe, Kind::SW | Kind::NW | Kind::EW | Kind::ST) {
                        paths.push((i, j + 1));
                    }
                }
            }
            Kind::SW => {
                if let Some(pipe) = kinds.get(i + 1).and_then(|v| v.get(j)) {
                    if matches!(pipe, Kind::NW | Kind::NE | Kind::NS | Kind::ST) {
                        paths.push((i + 1, j));
                    }
                }
                if j > 0 {
                    if let Some(pipe) = kinds.get(i).and_then(|v| v.get(j - 1)) {
                        if matches!(pipe, Kind::NE | Kind::SE | Kind::EW | Kind::ST) {
                            paths.push((i, j - 1));
                        }
                    }
                }
            }
            Kind::NS => {
                if i > 0 {
                    if let Some(pipe) = kinds.get(i - 1).and_then(|v| v.get(j)) {
                        if matches!(pipe, Kind::SW | Kind::SE | Kind::NS | Kind::ST) {
                            paths.push((i - 1, j));
                        }
                    }
                }
                if let Some(pipe) = kinds.get(i + 1).and_then(|v| v.get(j)) {
                    if matches!(pipe, Kind::NW | Kind::NE | Kind::NS | Kind::ST) {
                        paths.push((i + 1, j));
                    }
                }
            }
        }

        Self {
            kind,
            paths,
            coord: (i, j),
        }
    }
}
