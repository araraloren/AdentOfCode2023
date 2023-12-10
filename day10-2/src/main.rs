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

                if is_cycle {
                    println!("FOND");
                    break;
                }
                stack.push((next_pipe, 0));
            }
        }
    }

    println!("length = {}", stack.len());
    for pipe in &stack {
        println!("PATH: {pipe:?}");
    }

    let mut inside = vec![];

    for (i, line) in pipes.iter().enumerate() {
        for (j, pipe) in line.iter().enumerate() {
            if !stack.iter().any(|(v, _)| v.coord == pipe.coord) {
                let mut crossed = stack
                    .iter()
                    .filter(|(v, _)| v.coord.0 == i && v.coord.1 > j)
                    .collect::<Vec<_>>();

                if crossed.iter().all(|(v, _)| matches!(v.kind, Kind::NS)) {
                    if crossed.len() % 2 == 1 {
                        inside.push(pipe.clone());
                    }
                } else {
                    crossed.sort_by(|a, b| a.0.coord.1.partial_cmp(&b.0.coord.1).unwrap());
                    let mut count = 0;
                    let mut last = None;

                    for pipe in &crossed {
                        if matches!(pipe.0.kind, Kind::NS) {
                            count += 1;
                        } else {
                            match pipe.0.kind {
                                Kind::NE | Kind::SE => {
                                    if last.is_none() {
                                        last = Some(pipe);
                                    }
                                }
                                Kind::NW => {
                                    if let Some(last_pp) = last {
                                        if matches!(last_pp.0.kind, Kind::NE) {
                                            count += 2;
                                        } else {
                                            count += 1;
                                        }
                                        last = None;
                                    }
                                }
                                Kind::SW => {
                                    if let Some(last_pp) = last {
                                        if matches!(last_pp.0.kind, Kind::SE) {
                                            count += 2;
                                        } else {
                                            count += 1;
                                        }
                                        last = None;
                                    }
                                }
                                Kind::ST => {
                                    if let Some(_) = last {
                                        count += 1;
                                        last = None;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    if count % 2 == 1 {
                        println!("--+++++++> {:?} {:?}", (i, j), crossed);
                        inside.push(pipe.clone());
                    }
                }
            }
        }
    }

    println!("--> {:?} len = {}", inside, inside.len());

    for (i, line) in pipes.iter().enumerate() {
        for (j, pipe) in line.iter().enumerate() {
            if stack.iter().any(|(v, _)| v.coord == pipe.coord) {
                print!("{}", pipe.kind.to_str());
            } else if inside.iter().any(|v| v.coord == pipe.coord) {
                print!("?");
            } else {
                print!("+");
            }
        }
        println!("");
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

    pub fn to_str(&self) -> &str {
        match self {
            Kind::GN => ".",
            Kind::EW => "-",
            Kind::NE => "L",
            Kind::NW => "J",
            Kind::SE => "F",
            Kind::SW => "7",
            Kind::NS => "|",
            Kind::ST => "S",
        }
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
