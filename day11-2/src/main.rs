use neure::{err, prelude::*};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = '.'
        .or('#')
        .repeat_one()
        .map(Cell::new)
        .repeat(1..)
        .sep("\n");
    let mut image = RegexCtx::new(INPUT).ctor(&image)?;

    for (i, line) in image.iter_mut().enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            cell.coord = (i, j);
        }
    }
    let times = 1000000 - 1;
    let mut i = 0;

    while i < image.len() {
        if let Some(line) = image.get(i) {
            if line.iter().all(|v| v.is_space()) {
                for expand_line in image.iter_mut().skip(i) {
                    for cell in expand_line.iter_mut() {
                        if !cell.is_space() {
                            cell.expand_x(times);
                            println!("expand at {:?} at line {i}", cell);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    let mut j = 0;

    while j < image[0].len() {
        if image
            .iter()
            .all(|v| v.get(j).map(|v| v.is_space()).unwrap_or_default())
        {
            for expand_line in image.iter_mut() {
                for cell in expand_line.iter_mut().skip(j) {
                    if !cell.is_space() {
                        cell.expand_y(times);
                        println!("expand at {:?} at line {j}", cell);
                    }
                }
            }
        }
        j += 1;
    }
    let mut galaxies = vec![];
    let mut steps = 0;

    for (i, line) in image.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if !cell.is_space() {
                galaxies.push((i, j));
            }
        }
    }
    for (i, galaxy_i) in galaxies.iter().enumerate() {
        for (j, galaxy_j) in galaxies.iter().enumerate() {
            if j > i {
                let galaxy_i = image
                    .get(galaxy_i.0)
                    .and_then(|v| v.get(galaxy_i.1))
                    .unwrap();
                let galaxy_j = image
                    .get(galaxy_j.0)
                    .and_then(|v| v.get(galaxy_j.1))
                    .unwrap();
                let galaxy_i = galaxy_i.coord;
                let galaxy_j = galaxy_j.coord;
                let step = galaxy_i.0.abs_diff(galaxy_j.0) + galaxy_i.1.abs_diff(galaxy_j.1);

                println!(
                    "path@{i} <--> {j}: {:?} <-> {:?} = {}",
                    galaxy_i, galaxy_j, step
                );
                steps += step;
            }
        }
    }
    println!("--> got all steps = {steps}");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    kind: Kind,
    coord: (usize, usize),
}

impl Cell {
    pub fn new(val: &str) -> Result<Self, err::Error> {
        Ok(Self {
            kind: match val {
                "." => Kind::Space,
                _ => Kind::Galaxy,
            },
            coord: (0, 0),
        })
    }

    pub fn new_space() -> Self {
        Self {
            kind: Kind::Space,
            coord: (0, 0),
        }
    }

    pub fn is_space(&self) -> bool {
        matches!(self.kind, Kind::Space)
    }

    pub fn expand_x(&mut self, offset: usize) {
        self.coord.0 += offset;
    }

    pub fn expand_y(&mut self, offset: usize) {
        self.coord.1 += offset;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Space,

    Galaxy,
}
