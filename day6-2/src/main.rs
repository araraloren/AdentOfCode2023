use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sp = " ".ws();
    let num = neu::digit(10)
        .repeat_one_more()
        .sep(sp)
        .map(|v: Vec<&str>| {
            let mut ret = String::new();
            v.iter().for_each(|v| ret.push_str(v));
            Ok(ret)
        })
        .map(re::map::from_str_radix::<i64>(10));
    let time = "Time".sep_once(":".ws(), num.clone())._1();
    let dist = "Distance".sep_once(":".ws(), num)._1();
    let re = time.sep_once("\n".ws(), dist);
    let (time, dist) = CharsCtx::new(INPUT).ctor(&re)?;

    let kind = std::iter::once(time)
        .zip(std::iter::once(dist))
        .map(|(time, dist)| (1..time).filter(|hold| hold * (time - hold) > dist).count())
        .collect::<Vec<_>>();

    println!("--> get {:?} --> {}", kind, kind.iter().product::<usize>());

    Ok(())
}
