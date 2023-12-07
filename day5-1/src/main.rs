use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nl = '\n'.repeat_one();
    let nums = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<i64>(10))
        .sep(" ".ws());
    let seeds = "seeds".sep_once(":".ws(), nums);
    let name = neu::ascii_alphabetic().or('-').repeat_one_more();
    let map = name.sep_once(" ".ws(), "map".sep_once(":".ws(), nums.sep(nl.ws()))._1());
    let maps = map.sep(nl.ws());
    let regex = seeds.sep_once(nl.ws(), maps);
    let ((_, mut seeds), maps) = CharsCtx::new(INPUT).ctor(&regex)?;

    seeds.iter_mut().for_each(|dst| {
        println!("for seed {dst}");
        maps.iter().for_each(|(name, map)| {
            *dst = map
                .iter()
                .find(|range| *dst >= range[1] && *dst < range[1] + range[2])
                .map(|range| *dst - range[1] + range[0])
                .unwrap_or(*dst);
            println!("map in {} --> {}", name, *dst);
        })
    });

    println!("--> got {:?}, min is {:?}", seeds, seeds.iter().min());

    Ok(())
}
