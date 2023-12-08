use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nl = '\n'.repeat_one();
    let num = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<i64>(10));
    let re = "seeds"
        .sep_once(":".ws(), num.sep_once(" ".ws(), num).sep(" ".ws()))
        ._1();
    let re = re.sep_once(
        nl.ws(),
        neu::ascii_alphabetic()
            .or('-')
            .repeat_one_more()
            .sep_once(
                " ".ws(),
                "map"
                    .sep_once(":".ws(), num.sep(" ".ws()).sep(nl.ws()))
                    ._1(),
            )
            .sep(nl.ws()),
    );
    let (seeds, maps) = CharsCtx::new(INPUT).ctor(&re)?;
    let locs = seeds
        .into_iter()
        .map(|(dst, len)| {
            println!("for seed {dst}");
            (dst..dst + len)
                .map(|mut dst| {
                    maps.iter().for_each(|(_, map)| {
                        dst = map
                            .iter()
                            .find(|range| dst >= range[1] && dst < range[1] + range[2])
                            .map(|range| dst - range[1] + range[0])
                            .unwrap_or(dst);
                    });
                    dst
                })
                .min()
        })
        .collect::<Vec<_>>();

    println!("--> got {:?}, min is {:?}", locs, locs.iter().min());

    Ok(())
}
