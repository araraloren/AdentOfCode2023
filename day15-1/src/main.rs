use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = ','
        .not()
        .repeat_one_more()
        .map(calc_hash)
        .sep(",")
        .quote(re::start(), re::end());
    let seq = RegexCtx::new(INPUT).ctor(&re)?;

    dbg!(seq.iter().sum::<u32>());

    Ok(())
}

pub fn calc_hash(v: &str) -> Result<u32, neure::err::Error> {
    Ok(v.chars().fold(0, |a, ch| (a + ch as u32) * 17 % 256))
}
