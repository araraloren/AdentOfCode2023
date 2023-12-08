use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let space = neu::whitespace().repeat_full();

    let digit = neu::digit(10).repeat_one_more();
    let digit = digit.map(re::map::from_str_radix::<i32>(10));

    let re = digit
        .padded("Game ")
        .sep_once(
            ":".pad(space),
            digit
                .sep_once(" ", "blue".or("red").or("green"))
                .sep(",".pad(space))
                .sep(";".pad(space)),
        )
        .pad(space.or(re::null()))
        .repeat(1..);

    let sum: i32 = CharsCtx::new(INPUT)
        .ctor(&re)?
        .iter()
        .filter(|(_, sets)| {
            !sets.iter().any(|set| {
                set.iter().any(|(n, ty)| match *ty {
                    "red" => *n > 12,
                    "blue" => *n > 14,
                    "green" => *n > 13,
                    _ => panic!(""),
                })
            })
        })
        .map(|(id, _)| id)
        .sum();

    dbg!(sum);

    Ok(())
}
