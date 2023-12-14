use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<usize>(10))
        .sep(",");
    let springs = "."
        .map(|_| Ok(Record::Good))
        .or("#".map(|_| Ok(Record::Bad)))
        .or("?".map(|_| Ok(Record::Other)))
        .repeat(1..);
    let regex = springs.sep_once(" ".ws(), numbers).sep("\n");
    let records = RegexCtx::new(INPUT).ctor(&regex)?;
    let mut sum = 0;

    for record in records {
        let (springs, numbers) = record;
        let number = numbers.iter().sum::<usize>();
        let bad = springs.iter().filter(|v| v.is_bad()).count();
        let other: Vec<_> = springs
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_other())
            .map(|(i, _)| i)
            .collect();
        let need = number - bad;

        if need > 0 {
            let mut comb_count = 0;

            for comb in calc_combinations(other.len(), need) {
                let mut maybe = springs.clone();

                for i in &comb {
                    maybe[other[*i - 1]] = Record::Bad;
                }
                let mut failed = false;
                let mut iter = maybe.iter();

                for number in numbers.iter() {
                    let mut maybe_n = 0;

                    for v in iter.by_ref() {
                        if matches!(v, Record::Bad) {
                            maybe_n += 1;
                            break;
                        }
                    }
                    while matches!(iter.next(), Some(&Record::Bad)) {
                        maybe_n += 1;
                    }
                    if maybe_n != *number {
                        failed = true;
                        break;
                    }
                }
                if !failed && iter.all(|v| !v.is_bad()) {
                    comb_count += 1;
                }
            }
            println!("----------------------------------> {comb_count}");
            sum += comb_count;
        } else {
            sum += 1;
        }
    }

    println!("--> {sum}");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Record {
    Bad,
    Good,
    Other,
}

impl Record {
    pub fn is_good(&self) -> bool {
        matches!(self, Self::Good)
    }

    pub fn is_bad(&self) -> bool {
        matches!(self, Self::Bad)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other)
    }
}

pub fn calc_combinations(n: usize, c: usize) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    let (beg, end) = (0, c);
    let mut number: Vec<_> = (1..=c).collect();

    ret.push(number.clone());
    while number[beg] != n - c + 1 {
        let mut mt = end;

        loop {
            mt -= 1;
            if number[mt] != n - (end - mt) + 1 {
                number[mt] += 1;
                break;
            }
        }
        while mt + 1 != end {
            mt += 1;
            number[mt] = number[mt - 1] + 1;
        }
        ret.push(number.clone());
    }
    ret
}
