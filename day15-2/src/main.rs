use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add = neu::ascii_digit()
        .repeat_one()
        .map(re::map::from_str::<usize>())
        .padded("=")
        .map(|v| Ok(Some(v)));
    let re = neu::ascii_alphabetic()
        .repeat_one_more()
        .map(label)
        .then(add.or("-".map(|_| Ok(None))))
        .sep(",")
        .quote(re::start(), re::end());
    let seq = RegexCtx::new(INPUT).ctor(&re)?;
    let mut boxes = vec![vec![]; 256];

    seq.into_iter().for_each(|((label, id), op)| {
        let r#box = &mut boxes[id];

        if let Some(focal) = op {
            if let Some(lens) = r#box.iter_mut().find(|(v, _)| *v == label) {
                *lens = (label, focal);
            } else {
                r#box.push((label, focal));
            }
        } else if let Some(pos) = r#box.iter().position(|(v, _)| *v == label) {
            for i in pos + 1..r#box.len() {
                r#box[i - 1] = r#box[i];
            }
            r#box.pop();
        }
    });

    let power = boxes
        .iter()
        .enumerate()
        .map(|(id, r#box)| {
            r#box
                .iter()
                .enumerate()
                .fold(0, |a, (j, (_, focal))| a + (id + 1) * (j + 1) * focal)
        })
        .sum::<usize>();

    dbg!(power);

    Ok(())
}

pub fn label(v: &str) -> Result<(&str, usize), neure::err::Error> {
    Ok((
        v,
        v.chars().fold(0usize, |a, ch| (a + ch as usize) * 17 % 256),
    ))
}
