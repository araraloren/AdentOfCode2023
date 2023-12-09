use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sp = " ".ws();
    let num = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<i64>(10))
        .sep(sp);
    let time = "Time".sep_once(":".ws(), num)._1();
    let dist = "Distance".sep_once(":".ws(), num)._1();
    let re = time.sep_once("\n".ws(), dist);
    let (time, dist) = CharsCtx::new(INPUT).ctor(&re)?;

    let kind = time
        .into_iter()
        .zip(dist)
        .map(|(time, dist)| (1..time).filter(|hold| hold * (time - hold) > dist).count())
        .collect::<Vec<_>>();

    println!("--> get {:?} --> {}", kind, kind.iter().product::<usize>());

    Ok(())
}
