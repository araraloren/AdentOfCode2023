use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const NUMBER: usize = 1000000000;

    let re = "."
        .map(|_| Ok(Rock::Space))
        .or("#".map(|_| Ok(Rock::Cube)))
        .or("O".map(|_| Ok(Rock::Round)))
        .repeat(1..)
        .sep("\n");
    let mut platform = RegexCtx::new(INPUT).ctor(&re)?;
    let mut stack = vec![];

    for _ in 0.. {
        let (load, next) = calcuate_load(tilting_platform_cycle(platform));

        stack.push(load);
        platform = next;
        if let Some((beg, len)) = find_duplicated_sequence(&stack, 100) {
            let tot = stack.len();
            let idx = (tot - len) + NUMBER % len - beg - 1;

            println!("{stack:?} len = {}", tot);
            println!("duplciated sequence is {:?}", &stack[tot - len..tot]);
            println!("--> {}", stack[idx]);
            break;
        }
    }
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

pub fn find_duplicated_sequence(stack: &[usize], count: usize) -> Option<(usize, usize)> {
    let s_len = stack.len();
    let max_len = 1.max(s_len / count);

    if s_len >= count * max_len {
        for len in 1..=max_len {
            let mut found = true;

            'all: for cnt in 1..count {
                for idx in s_len - len..s_len {
                    if stack[idx] != stack[idx - cnt * len] {
                        found = false;
                        break 'all;
                    }
                }
            }
            if found {
                return Some((s_len - (count * len), len));
            }
        }
    }
    None
}

pub fn tilting_platform_cycle(platform: Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    tilting_platform(
        tilting_platform(
            tilting_platform(tilting_platform(platform, Dir::N), Dir::W),
            Dir::S,
        ),
        Dir::E,
    )
}

pub fn tilting_platform(mut platform: Vec<Vec<Rock>>, dir: Dir) -> Vec<Vec<Rock>> {
    let row = platform.len();
    let col = platform[0].len();
    let mut tilt = |i: usize, j: usize| {
        if platform[i][j].is_round() {
            let (mut k, mut l) = dir.next(i, j);
            let (k, l) = loop {
                if !platform[k][l].is_space() {
                    break dir.prev(k, l);
                }
                if dir.jump(k, l, row, col) {
                    break (k, l);
                } else {
                    (k, l) = dir.next(k, l);
                }
            };

            platform[i][j] = Rock::Space;
            platform[k][l] = Rock::Round;
        }
    };

    match dir {
        Dir::N => {
            for i in 1..row {
                for j in 0..col {
                    tilt(i, j);
                }
            }
        }
        Dir::W => {
            for i in 0..row {
                for j in 1..col {
                    tilt(i, j);
                }
            }
        }
        Dir::S => {
            for i in (0..row - 1).rev() {
                for j in 0..col {
                    tilt(i, j);
                }
            }
        }
        Dir::E => {
            for i in 0..row {
                for j in (0..col - 1).rev() {
                    tilt(i, j);
                }
            }
        }
    }
    platform
}

pub fn calcuate_load(platform: Vec<Vec<Rock>>) -> (usize, Vec<Vec<Rock>>) {
    let len = platform.len();

    (
        platform.iter().enumerate().fold(0, |sum, (i, line)| {
            sum + (len - i) * line.iter().filter(|v| v.is_round()).count()
        }),
        platform,
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    N,
    W,
    S,
    E,
}

impl Dir {
    pub fn jump(&self, i: usize, j: usize, row: usize, col: usize) -> bool {
        match self {
            Dir::N => i == 0,
            Dir::W => j == 0,
            Dir::S => i == row - 1,
            Dir::E => j == col - 1,
        }
    }

    pub fn next(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Dir::N => (i - 1, j),
            Dir::W => (i, j - 1),
            Dir::S => (i + 1, j),
            Dir::E => (i, j + 1),
        }
    }

    pub fn prev(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Dir::N => (i + 1, j),
            Dir::W => (i, j + 1),
            Dir::S => (i - 1, j),
            Dir::E => (i, j - 1),
        }
    }
}
