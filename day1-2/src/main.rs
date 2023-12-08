const INPUT: &str = include_str!("../input.txt");

use neure::{prelude::*, re::DynamicCreateCtorThenHelper};

fn main() {
    let letters = "one"
        .map(|_| Ok(1))
        .or("two".map(|_| Ok(2)))
        .or("three".map(|_| Ok(3)))
        .or("four".map(|_| Ok(4)))
        .or("five".map(|_| Ok(5)))
        .or("six".map(|_| Ok(6)))
        .or("seven".map(|_| Ok(7)))
        .or("eight".map(|_| Ok(8)))
        .or("nine".map(|_| Ok(9)));
    let digit = neu::digit(10)
        .repeat_one()
        .map(re::map::from_str_radix::<i32>(10));
    let line_re = letters
        .dyn_then_ctor(|val: &i32| {
            let index = (*val as usize).saturating_sub(1);
            Ok(move |ctx: &mut CharsCtx| {
                ctx.dec(*[2, 2, 4, 3, 3, 2, 4, 4, 3].get(index).unwrap_or(&0));
                Ok(Span::new(ctx.offset(), 0))
            })
        })
        ._0()
        .or(digit)
        .padded(
            neu::ascii_alphabetic()
                .repeat_zero_more()
                .set_cond(neu::re_cond(re::not(letters.or(digit)))),
        )
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
        println!("--> {value}");
        sum += value;
    }
    dbg!(sum);
}
