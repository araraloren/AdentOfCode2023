use std::collections::HashMap;

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sp = neu::whitespace().repeat_one_more();
    let ins = 'R'.or('L').repeat_one().map(I::new).repeat(1..);
    let ele = neu::ascii_alphabetic().repeat_times::<3>();
    let map = ele
        .sep_once(
            "=".pad(sp).padded(sp),
            ele.sep_once(",".ws(), ele).quote("(", ")"),
        )
        .sep_collect("\n".ws());
    let re = ins.sep_once(sp, map);
    let (ins, map): (_, HashMap<&str, (&str, &str)>) = CharsCtx::new(INPUT).ctor(&re)?;
    let mut cur: &str = "AAA";
    let mut step = 0;

    if ins.iter().cycle().any(|ins| match (ins, map.get(cur)) {
        (I::L, Some(("ZZZ", _))) | (I::R, Some((_, "ZZZ"))) => true,
        (I::L, Some((l, _))) | (I::R, Some((_, l))) => {
            cur = l;
            step += 1;
            false
        }
        (_, None) => {
            panic!("Can not find key {cur:?}")
        }
    }) {
        println!("--> {}", step + 1);
    }

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
