use std::cmp::Ordering;

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line = ".".or("#").repeat(1..).sep("\n").sep("\n");
    let mut maps = RegexCtx::new(INPUT).ctor(&line)?;
    let mut sum = 0;

    for (_, map) in maps.iter_mut().enumerate() {
        let mut is_h = true;
        let mut old = find_reflecttion_h(map);

        if old.is_empty() {
            is_h = false;
            old = find_reflecttion_v(map);
        }
        'next_map: for i in 0..map.len() {
            for j in 0..map[0].len() {
                map[i][j] = if map[i][j] == "." { "#" } else { "." };
                for h in find_reflecttion_h(map) {
                    if !is_h || !old.contains(&h) {
                        sum += (h.0 - h.1 - h.2) / 2 + h.1;
                        break 'next_map;
                    }
                }
                for v in find_reflecttion_v(map) {
                    if is_h || !old.contains(&v) {
                        sum += 100 * ((v.0 - v.1 - v.2) / 2 + v.1);
                        break 'next_map;
                    }
                }
                map[i][j] = if map[i][j] == "." { "#" } else { "." };
            }
        }
    }

    println!("--> sum = {}", sum);

    Ok(())
}

pub fn find_reflecttion_h(map: &[Vec<&str>]) -> Vec<(usize, usize, usize)> {
    let mut ret = vec![];
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
        found.then_some((len, beg, end))
    };

    (0..len - 1).for_each(|v| ret.extend(finder(0, v).into_iter().chain(finder(v, 0))));
    ret
}

pub fn find_reflecttion_v(map: &[Vec<&str>]) -> Vec<(usize, usize, usize)> {
    let mut ret = vec![];
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
        found.then_some((len, beg, end))
    };

    (0..len - 1).for_each(|v| ret.extend(finder(0, v).into_iter().chain(finder(v, 0))));
    ret
}
