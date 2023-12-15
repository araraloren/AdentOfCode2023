use std::cmp::Ordering;

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line = ".".or("#").repeat(1..).sep("\n").sep("\n");
    let maps = RegexCtx::new(INPUT).ctor(&line)?;
    let mut sum = 0;

    for (idx, map) in maps.iter().enumerate() {
        if let Some(h) = find_reflection_h(map) {
            println!("map {idx} is a h reflection --> {}", h);
            sum += h;
        } else if let Some(v) = find_reflecttion_v(map) {
            println!("map {idx} is a v reflection --> {}", v);
            sum += 100 * v;
        }
        println!("Map {idx} --------------------------------");
    }

    println!("--> sum = {}", sum);

    Ok(())
}

pub fn find_reflection_h(map: &[Vec<&str>]) -> Option<usize> {
    let len = map[0].len();
    let finder = |beg: usize, end: usize| {
        let mut found = true;

        'search: for line in map.iter() {
            for ((i, l), (j, r)) in line
                .iter()
                .enumerate()
                .skip(beg)
                .zip(line.iter().enumerate().rev().skip(end))
            {
                if (i < j && l != r) || i == j {
                    found = false;
                    break 'search;
                } else if i > j {
                    break;
                }
            }
        }
        found.then_some((len - beg - end) / 2 + beg)
    };

    for v in 0..len - 1 {
        if let Some(v) = finder(0, v) {
            return Some(v);
        } else if let Some(v) = finder(v, 0) {
            return Some(v);
        }
    }
    None
}

pub fn find_reflecttion_v(map: &[Vec<&str>]) -> Option<usize> {
    let len = map.len();
    let finder = |beg: usize, end: usize| {
        let mut found = true;

        'search: for ((i, u), (j, d)) in map
            .iter()
            .enumerate()
            .skip(beg)
            .zip(map.iter().enumerate().rev().skip(end))
        {
            match i.cmp(&j) {
                Ordering::Less => {
                    for (a, b) in u.iter().zip(d.iter()) {
                        if a != b {
                            found = false;
                            break 'search;
                        }
                    }
                }
                Ordering::Equal => {
                    found = false;
                    break 'search;
                }
                Ordering::Greater => {
                    break;
                }
            }
        }
        found.then_some((len - beg - end) / 2 + beg)
    };

    for v in 0..len - 1 {
        if let Some(v) = finder(0, v) {
            return Some(v);
        } else if let Some(v) = finder(v, 0) {
            return Some(v);
        }
    }
    None
}
