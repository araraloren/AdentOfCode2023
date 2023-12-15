use std::fmt::Debug;

use neure::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let number = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<usize>(10))
        .sep(",");
    let spring = "."
        .map(|_| Ok(Record::Good))
        .or("#".map(|_| Ok(Record::Bad)))
        .or("?".map(|_| Ok(Record::Mark)))
        .repeat(1..);
    let regex = spring.sep_once(" ".ws(), number).sep("\n");
    let records = RegexCtx::new(INPUT).ctor(&regex)?;
    let n = 5;
    let records: Vec<_> = records
        .iter()
        .map(|(spring, number)| {
            let mut left = vec![];
            let mut right = vec![];

            for i in 0..n {
                left.extend(spring.clone());
                right.extend(number.clone());
                if i != n - 1 {
                    left.push(Record::Mark);
                }
            }
            (left, right)
        })
        .collect();

    process_record(records);

    Ok(())
}

pub fn process_record(records: Vec<(Vec<Record>, Vec<usize>)>) {
    let check_bad = |slice: Option<&[Record]>, len: usize| {
        if let Some(slice) = slice {
            if slice.len() >= len {
                return slice
                    .get(len)
                    .map(|v| v.is_good() || v.is_mark())
                    .unwrap_or_default()
                    && (0..len).all(|v| {
                        slice
                            .get(v)
                            .map(|v| v.is_bad() || v.is_mark())
                            .unwrap_or_default()
                    });
            }
        }
        false
    };

    let mut sum = 0;

    for record in records {
        let (mut spring, mut number) = record;
        let (row, col) = (number.len() + 1, spring.len() + 2);
        let mut table = vec![vec![0usize; col]; row];

        number.push(0);
        number.reverse();
        spring.push(Record::Good);
        for j in (0..col - 1).rev() {
            if spring[j].is_mark() || spring[j].is_good() {
                table[0][j + 1] = 1;
            } else {
                break;
            }
        }
        for i in 1..row {
            let num = number[i];

            for j in (0..col - 2).rev() {
                let item = spring[j];
                let count = match item {
                    Record::Good => table[i][j + 1],
                    Record::Mark | Record::Bad => {
                        let mut count = 0;

                        if check_bad(spring.get(j..), num) {
                            count += table[i - 1][j + num + 1]
                        }
                        if item.is_mark() {
                            count += table[i][j + 1]
                        }
                        count
                    }
                };
                table[i][j] = count;
            }
        }

        for item in &table {
            println!("--> {item:?}");
        }
        sum += table[row - 1][0];
    }
    println!("--> sum = {sum}");
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Record {
    Bad,
    Good,
    Mark,
}

impl Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bad => write!(f, "Bad"),
            Self::Good => write!(f, "Good"),
            Self::Mark => write!(f, "Mark"),
        }
    }
}

impl Record {
    pub fn is_good(&self) -> bool {
        matches!(self, Self::Good)
    }

    pub fn is_bad(&self) -> bool {
        matches!(self, Self::Bad)
    }

    pub fn is_mark(&self) -> bool {
        matches!(self, Self::Mark)
    }
}
