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
        .map(|(_, sets)| {
            let (mut red, mut blue, mut green) = (0, 0, 0);
            sets.iter().for_each(|set| {
                set.iter().for_each(|(n, ty)| match *ty {
                    "red" => red = red.max(*n),
                    "blue" => blue = blue.max(*n),
                    "green" => green = green.max(*n),
                    _ => panic!(""),
                })
            });
            (red, blue, green)
        })
        .map(|(red, blue, green)| red * blue * green)
        .sum();

    dbg!(sum);

    Ok(())
}