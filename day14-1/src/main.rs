use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = "."
        .map(|_| Ok(Rock::Space))
        .or("#".map(|_| Ok(Rock::Cube)))
        .or("O".map(|_| Ok(Rock::Round)))
        .repeat(1..)
        .sep("\n");
    let mut platform = RegexCtx::new(INPUT).ctor(&re)?;

    tilting_platform(&mut platform);
    dbg!(calcuate_load(&platform));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rock {
    Round,
    Cube,
    Space,
}

impl Rock {
    pub fn is_round(&self) -> bool {
        matches!(self, Self::Round)
    }

    pub fn is_cube(&self) -> bool {
        matches!(self, Self::Cube)
    }

    pub fn is_space(&self) -> bool {
        matches!(self, Self::Space)
    }
}

pub fn tilting_platform(platform: &mut Vec<Vec<Rock>>) {
    for i in 1..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j].is_round() {
                let mut k = i - 1;
                let k = loop {
                    if !platform[k][j].is_space() {
                        break k + 1;
                    }
                    if k == 0 {
                        break 0;
                    } else {
                        k -= 1;
                    }
                };

                platform[i][j] = Rock::Space;
                platform[k][j] = Rock::Round;
            }
        }
    }
}

pub fn calcuate_load(platform: &[Vec<Rock>]) -> usize {
    let len = platform.len();

    platform.iter().enumerate().fold(0, |sum, (i, line)| {
        sum + (len - i) * line.iter().filter(|v| v.is_round()).count()
    })
}
