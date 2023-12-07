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
    let mut counts = vec![1; cards.len()];

    cards
        .iter()
        .map(|(win, nums)| win.iter().filter(|v| nums.contains(v)).count())
        .enumerate()
        .for_each(|(i, c)| {
            (1..=c).for_each(|c| {
                counts[c + i] += counts[i];
            })
        });

    println!("got {:?} --> {}", counts, counts.iter().sum::<i32>());
    Ok(())
}
