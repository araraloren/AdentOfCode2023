use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = neu::digit(10)
        .or('-')
        .repeat_one_more()
        .map(re::map::from_str_radix::<i64>(10))
        .sep(" ".ws());
    let re = num.sep("\n".ws());
    let lines = CharsCtx::new(INPUT).ctor(&re)?;
    let total = lines
        .iter()
        .map(|line| {
            let mut diffs = vec![line.clone()];
            let val = loop {
                if let Some(last) = diffs.last() {
                    if last.iter().all(|v| *v == 0) {
                        break diffs.iter().enumerate().fold(0, |a, (id, val)| {
                            if id % 2 == 0 {
                                a + val.first().unwrap()
                            } else {
                                a - val.first().unwrap()
                            }
                        });
                    } else {
                        let mut next = Vec::with_capacity(last.len());
                        for (l, r) in last.iter().zip(last.iter().skip(1)) {
                            next.push(r - l);
                        }
                        diffs.push(next);
                    }
                }
            };

            println!("--> {:?} previous is {}", line, val);
            val
        })
        .sum::<i64>();

    println!("--> total is {}", total);

    Ok(())
}
