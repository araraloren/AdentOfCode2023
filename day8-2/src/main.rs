use std::collections::HashMap;

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sp = neu::whitespace().repeat_one_more();
    let ins = 'R'.or('L').repeat_one().map(I::new).repeat(1..);
    let ele = neu::ascii_alphanumeric().repeat_times::<3>();
    let map = ele
        .sep_once(
            "=".pad(sp).padded(sp),
            ele.sep_once(",".ws(), ele).quote("(", ")"),
        )
        .sep_collect("\n".ws());
    let re = ins.sep_once(sp, map);
    let (ins, map): (_, HashMap<&str, (&str, &str)>) = CharsCtx::new(INPUT).ctor(&re)?;
    let curs = map
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(v, _)| {
            println!("find start with {:?}", v);
            v
        })
        .cloned()
        .collect::<Vec<_>>();
    let steps = curs
        .into_iter()
        .map(|mut cur| {
            let mut step = 0usize;

            if ins.iter().cycle().any(|ins| {
                match (ins, map.get(cur)) {
                    (I::L, Some((l, _))) | (I::R, Some((_, l))) => {
                        cur = *l;
                    }
                    (_, None) => {
                        panic!("Can not find key {cur:?}")
                    }
                }
                step += 1;
                cur.ends_with('Z')
            }) {
                println!("--> for ele {cur:?} found end in {}", step);
            }
            step
        })
        .collect::<Vec<_>>();

    println!("---> {}", steps.iter().fold(1, |a, b| a * *b / gcd(a, *b)));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum I {
    L,
    R,
}

impl I {
    pub fn new(val: &str) -> Result<I, neure::err::Error> {
        assert!(matches!(val, "L" | "R"));
        Ok(match val {
            "R" => Self::R,
            _ => Self::L,
        })
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut num = a.max(b);
    let mut div = a.min(b);

    loop {
        let rem = num % div;

        if rem == 0 {
            break div;
        } else {
            num = div;
            div = rem;
        }
    }
}
