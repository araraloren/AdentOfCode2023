use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = "Card".ws().then(neu::digit(10).repeat_one_more());
    let nums = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<i32>(10))
        .sep(" ".ws());
    let nums = nums.sep_once("|".ws(), nums);
    let regex = title.sep_once(":".ws(), nums)._1();
    let cards = INPUT
        .lines()
        .map(|v| CharsCtx::new(v).ctor(&regex))
        .collect::<Result<Vec<(Vec<_>, Vec<_>)>, _>>()?;

    let ret = cards
        .iter()
        .map(|(win, nums)| win.iter().filter(|v| nums.contains(v)).count() as u32)
        .filter(|v| *v > 0)
        .map(|v| 2_u32.pow(v - 1))
        .collect::<Vec<_>>();

    println!("got {:?} --> {}", &ret, ret.iter().sum::<u32>());
    Ok(())
}
