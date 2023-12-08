use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    enum T {
        I,
        S(usize),
        N((usize, usize, i32)),
    }
    let digit = neu::digit(10).repeat_one_more();
    let digit =
        digit.map(|(s, v): (Span, &str)| Ok(T::N((s.beg, s.len, v.parse::<i32>().unwrap()))));
    let dots = '.'.repeat_zero_more();
    let re = digit
        .or('*'.repeat_one().map(|s: (Span, _)| Ok(T::S(s.0.beg))))
        .or('.'.not().repeat_one().map(|_| Ok(T::I)))
        .pad(dots)
        .padded(dots)
        .repeat(1..);

    let map = INPUT
        .lines()
        .map(|v| CharsCtx::new(v).ctor_with(&re, &mut |s, v| Ok((s, v))))
        .collect::<Result<Vec<Vec<_>>, _>>()?;

    let mut sum = 0;
    let get_val = |gear, other| {
        if let &T::N((beg, len, val)) = other {
            if gear >= beg.saturating_sub(1) && gear <= (beg + len) {
                return Some(val);
            }
        }
        None
    };

    for (i, line) in map.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if let T::S(gear) = val {
                let mut parts = vec![];

                for k in [j.saturating_sub(1), j + 1] {
                    if let Some(other) = line.get(k) {
                        if let Some(val) = get_val(*gear, other) {
                            parts.push(val);
                        }
                    }
                }
                for k in [i.saturating_sub(1), i + 1] {
                    if let Some(line) = map.get(k) {
                        parts.extend(line.iter().filter_map(|v| get_val(*gear, v)));
                    }
                }
                if parts.len() >= 2 {
                    println!("--> {:?} {:?} is a part number", val, parts);
                    sum += parts.iter().product::<i32>();
                }
            }
        }
    }
    println!("--> {sum}");

    Ok(())
}
