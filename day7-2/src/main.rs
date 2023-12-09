use std::collections::HashMap;

use neure::{err, prelude::*};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hand = neu::ascii_alphanumeric()
        .repeat_one()
        .map(Card::new)
        .repeat(5..6)
        .map(Hand::new);
    let bid = neu::digit(10)
        .repeat_one_more()
        .map(re::map::from_str_radix::<usize>(10));
    let re = hand.sep_once(" ", bid).sep("\n".ws());
    let mut input = CharsCtx::new(INPUT).ctor(&re)?;

    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for item in &input {
        println!("--> get {:?}", item);
    }
    println!(
        "--> total = {}",
        input
            .iter()
            .enumerate()
            .map(|(idx, item)| item.1 * (idx + 1))
            .sum::<usize>()
    );

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Hand {
    ty: u8,
    card: Vec<Card>,
}

impl Hand {
    pub fn new(card: Vec<Card>) -> Result<Self, err::Error> {
        let mut hash_map: HashMap<Card, i32> = HashMap::default();

        card.iter()
            .for_each(|ch| *hash_map.entry(*ch).or_insert(0) += 1);
        let mut values = hash_map.values();
        let (any_a, any_b) = (values.next().unwrap(), values.next());
        let card_j = *hash_map.get(&Card::J).unwrap_or(&0);
        let ty = match hash_map.len() {
            1 => {
                7 /* five of a kind */
            }
            2 => {
                if card_j > 0 {
                    7 /* five of a kind */
                }
                else {
                    match any_a {
                        1 | 4 => {
                            6 /* four of a kind */
                        }
                        _ /* 2 | 3 */ => {
                            5 /* full house */
                        }
                    }
                }
            }
            3 => {
                let any_b = any_b.unwrap();

                match (any_a, any_b) {
                    (1, 2) | (2, _) => {
                        match card_j {
                            1 => {
                                5 /* full house */
                            }
                            2 => {
                                6 /* four of a kind */
                            }
                            _ => {
                                3 /* two pair */
                            }
                        }
                    }
                    _ /* (1, 1 | 3) | (3, _) */ => {
                        if card_j > 0 {
                            6 /* four of a kind */
                        }
                        else {
                            4 /* three of a kind */
                        }
                    }
                }
            }
            4 => {
                if card_j > 0 {
                    4 /* three of a kind */
                }
                else {
                    2 /* one pair */
                }
            }
            _ /* 5 */ => {
                if card_j > 0 {
                    2 /* one pair */
                }
                else {
                    1 /* high card */
                }
            }
        };

        Ok(Self { ty, card })
    }
}

impl PartialEq<Hand> for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.ty == other.ty && self.card == other.card
    }
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        match self.ty.partial_cmp(&other.ty) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for (l, r) in self.card.iter().zip(other.card.iter()) {
            match l.partial_cmp(r) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }
        Some(core::cmp::Ordering::Equal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl Card {
    pub fn new(card: &str) -> Result<Self, err::Error> {
        Ok(match card {
            "2" => Self::N2,
            "3" => Self::N3,
            "4" => Self::N4,
            "5" => Self::N5,
            "6" => Self::N6,
            "7" => Self::N7,
            "8" => Self::N8,
            "9" => Self::N9,
            "T" => Self::T,
            "J" => Self::J,
            "Q" => Self::Q,
            "K" => Self::K,
            _ /* "A" */ => Self::A,
        })
    }
}
