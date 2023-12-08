const INPUT: &str = include_str!("../input.txt");

use neure::prelude::*;

fn main() {
    let line_re = neu::digit(10)
        .repeat_one()
        .map(re::map::from_str_radix::<i32>(10))
        .padded(neu::ascii_alphabetic().repeat_zero_more())
        .repeat(1..)
        .map(|v: Vec<i32>| Ok(v[0] * 10 + v[v.len() - 1]))
        .pad(
            neu::any()
                .repeat_full()
                .set_cond(neu::re_cond(re::not("\r\n")))
                .then("\r\n".or(re::null())),
        );

    let mut sum = 0;
    let mut ctx = CharsCtx::new(INPUT);

    while let Ok(value) = ctx.ctor(&line_re) {
        sum += value;
    }

    dbg!(sum);
}
