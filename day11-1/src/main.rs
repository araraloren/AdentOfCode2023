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
    let mut i = 0;

    while i < image.len() {
        if let Some(line) = image.get(i) {
            if line.iter().all(|v| v.is_space()) {
                image.insert(i, vec![Cell::new_space(); line.len()]);
                i += 1;
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
            for line in image.iter_mut() {
                line.insert(j, Cell::new_space());
            }
            j += 1;
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
}

impl Cell {
    pub fn new(val: &str) -> Result<Self, err::Error> {
        Ok(Self {
            kind: match val {
                "." => Kind::Space,
                _ => Kind::Galaxy,
            },
        })
    }

    pub fn new_space() -> Self {
        Self { kind: Kind::Space }
    }

    pub fn is_space(&self) -> bool {
        matches!(self.kind, Kind::Space)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Space,

    Galaxy,
}
