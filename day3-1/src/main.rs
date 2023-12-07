use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    enum T {
        S(usize),
        N((usize, usize, i32)),
    }
    let digit = neu::digit(10).repeat_one_more();
    let digit =
        digit.map(|(s, v): (Span, &str)| Ok(T::N((s.beg, s.len, v.parse::<i32>().unwrap()))));
    let dots = '.'.repeat_zero_more();
    let re = digit
        .or('.'
            .not()
            .repeat_one()
            .map(|(s, _): (Span, _)| Ok(T::S(s.beg))))
        .pad(dots)
        .padded(dots)
        .repeat(1..);

    let map = INPUT
        .lines()
        .map(|v| CharsCtx::new(v).ctor_with(&re, &mut |s, v| Ok((s, v))))
        .collect::<Result<Vec<Vec<_>>, _>>()?;

    let mut sum = 0;

    for (i, line) in map.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if let T::N((beg, len, val)) = val {
                let (min, max) = (beg.saturating_sub(1), beg + len);
                let mut part = false;

                for k in [j.saturating_sub(1), j + 1] {
                    part = part
                        || matches!(line.get(k), Some(T::S(beg)) if *beg >= min && *beg <= max);
                }
                for k in [i.saturating_sub(1), i + 1] {
                    if let Some(line) = map.get(k) {
                        part = part
                            || line
                                .iter()
                                .any(|val| matches!(val, T::S(beg) if *beg >= min && *beg <= max));
                    }
                }
                if part {
                    println!("--> {} is a part number", val);
                    sum += val;
                }
            }
        }
    }
    println!("--> {sum}");

    Ok(())
}
