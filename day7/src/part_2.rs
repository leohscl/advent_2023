use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum Letter {
    J,
    Num(u32),
    T,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct FiveLetters {
    letter_1: Letter,
    letter_2: Letter,
    letter_3: Letter,
    letter_4: Letter,
    letter_5: Letter,
}

impl FromStr for FiveLetters {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut letter_iter = s.chars();
        let letter_1 = Letter::from_str(&letter_iter.next().unwrap().to_string())?;
        let letter_2 = Letter::from_str(&letter_iter.next().unwrap().to_string())?;
        let letter_3 = Letter::from_str(&letter_iter.next().unwrap().to_string())?;
        let letter_4 = Letter::from_str(&letter_iter.next().unwrap().to_string())?;
        let letter_5 = Letter::from_str(&letter_iter.next().unwrap().to_string())?;
        Ok(FiveLetters {
            letter_1,
            letter_2,
            letter_3,
            letter_4,
            letter_5,
        })
    }
}

use Letter::*;
impl FromStr for Letter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(A),
            "K" => Ok(K),
            "Q" => Ok(Q),
            "J" => Ok(J),
            "T" => Ok(T),
            _ => Ok(Num(s.parse::<u32>().unwrap())),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    HighCard(FiveLetters),
    Pair(FiveLetters),
    TwoPair(FiveLetters),
    ThreeOfAKind(FiveLetters),
    FullHouse(FiveLetters),
    FourOfAKind(FiveLetters),
    FiveOfAKind(FiveLetters),
}

use Hand::*;

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hash_letters = HashMap::new();
        s.chars().for_each(|char| {
            hash_letters
                .entry(char)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
        let opt_ref_num_jokers = hash_letters.get_mut(&'J');
        let mut num_jokers = 0;
        if let Some(ref_num_jokers) = opt_ref_num_jokers {
            num_jokers = *ref_num_jokers;
            *ref_num_jokers = 0;
        }
        let mut vec_values: Vec<i32> = hash_letters.values().cloned().collect();
        vec_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
        vec_values[0] = vec_values[0] + num_jokers;
        let five_letters = FiveLetters::from_str(&s.to_string()).unwrap();
        match (vec_values.get(0), vec_values.get(1)) {
            (Some(5), _) => Ok(FiveOfAKind(five_letters)),
            (Some(4), _) => Ok(FourOfAKind(five_letters)),
            (Some(3), Some(2)) => Ok(FullHouse(five_letters)),
            (Some(3), Some(1)) => Ok(ThreeOfAKind(five_letters)),
            (Some(2), Some(2)) => Ok(TwoPair(five_letters)),
            (Some(2), Some(1)) => Ok(Pair(five_letters)),
            (Some(1), Some(1)) => Ok(HighCard(five_letters)),
            _ => Err(()),
        }
    }
}

impl Hand {
    fn get_five_letters(&self) -> FiveLetters {
        let five_letters = match self {
            FiveOfAKind(five_letters) => five_letters,
            FourOfAKind(five_letters) => five_letters,
            FullHouse(five_letters) => five_letters,
            ThreeOfAKind(five_letters) => five_letters,
            TwoPair(five_letters) => five_letters,
            Pair(five_letters) => five_letters,
            HighCard(five_letters) => five_letters,
        };
        five_letters.clone()
    }
    fn compare_five_letters(&self, other: &Hand) -> std::cmp::Ordering {
        self.get_five_letters().cmp(&other.get_five_letters())
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
struct Play {
    hand: Hand,
    bid: u64,
}

impl FromStr for Play {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter_word = s.split_whitespace();
        let hand = Hand::from_str(iter_word.next().unwrap()).unwrap();
        let bid = iter_word.next().unwrap().parse::<u64>().unwrap();
        Ok(Play { hand, bid })
    }
}
use std::mem::discriminant;

fn main() {
    // let input = include_str!("./input_test.txt");
    let input = include_str!("./input.txt");
    let mut vec_play: Vec<_> = input
        .split_terminator("\n")
        .map(|line| Play::from_str(line).unwrap())
        .collect();
    vec_play.sort_by(|a, b| {
        if discriminant(&a.hand) == discriminant(&b.hand) {
            a.hand.compare_five_letters(&b.hand)
        } else {
            a.cmp(b)
        }
    });

    let winnings: u64 = vec_play
        .into_iter()
        .inspect(|play| {
            dbg!(play);
        })
        .enumerate()
        .map(|(rank, play)| (rank + 1) as u64 * play.bid)
        .sum();
    dbg!(winnings);
}
