use combine::{error::ParseError, parser::*, stream::RangeStream};
use std::collections::HashSet;

/// 1. Find the first winning bingo card
/// 2. Find the last winning bingo card
pub fn run(input: &str) -> (u32, u32) {
    // part one
    let (balls, mut cards) = parse(input);
    let caller = BingoCaller::new(balls);
    let mut winner: Option<BingoCard> = None;
    let mut first_score = 0;
    for (ball, board) in caller {
        for card in cards.iter_mut() {
            if card.mark(ball) {
                winner = Some(card.clone());
                break;
            }
        }
        if winner.is_some() {
            let points = if let Some(card) = winner {
                card.points(&board)
            } else {
                0
            };
            first_score = points * ball as u32;
            break;
        }
    }

    // part two
    let (balls, mut cards) = parse(input);
    let caller = BingoCaller::new(balls);
    let mut second_score = 0;
    for (ball, board) in caller {
        for card in cards.iter_mut() {
            if !card.won && card.mark(ball) {
                let points = card.points(&board);
                second_score = points * ball as u32;
            }
        }
    }

    (first_score, second_score)
}

struct BingoCaller {
    balls: Vec<u8>,
    index: usize,
}

impl BingoCaller {
    fn new(numbers: Vec<u8>) -> Self {
        BingoCaller {
            balls: numbers,
            index: 0,
        }
    }
}

impl Iterator for BingoCaller {
    type Item = (u8, HashSet<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.balls.len() {
            None
        } else {
            let ball = self.balls[self.index];
            let board: HashSet<u8> = self.balls[..=self.index].iter().cloned().collect();
            self.index += 1;
            Some((ball, board))
        }
    }
}

type Row = [u8; 5];
type Card = [Row; 5];

#[derive(Debug, Clone)]
struct BingoCard {
    rows: Card,
    mark_row: Vec<usize>,
    mark_col: Vec<usize>,
    won: bool,
}

impl BingoCard {
    fn new(rows: Card) -> Self {
        BingoCard {
            rows,
            mark_row: Vec::new(),
            mark_col: Vec::new(),
            won: false,
        }
    }

    /// Mark a card with latest number and return whether it's a winner or not.
    fn mark(&mut self, num: u8) -> bool {
        let mut winner = false;
        for (y, row) in self.rows.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == num {
                    self.mark_row.push(y);
                    self.mark_col.push(x);
                    if self.mark_row.len() >= 5 {
                        winner = win_check(&self.mark_row) || win_check(&self.mark_col);
                    }
                    break;
                }
            }
        }
        if winner {
            self.won = true;
        }
        winner
    }

    fn points(&self, board: &HashSet<u8>) -> u32 {
        let mut points = 0;
        for row in self.rows.iter() {
            for num in row.iter() {
                if !board.contains(num) {
                    points += *num as u32
                }
            }
        }
        points
    }
}

fn win_check(marks: &Vec<usize>) -> bool {
    let in_a_row = marks.iter().fold(Row::default(), |mut acc, &x| {
        acc[x] += 1;
        acc
    });
    in_a_row.iter().filter(|&&v| v == 5).count() >= 1
}

fn parse(input: &str) -> (Vec<u8>, Vec<BingoCard>) {
    let (balls, tail) = balls().easy_parse(input).unwrap();
    let (nums, _) = cards_nums().easy_parse(tail).unwrap();
    let cards: Vec<BingoCard> = nums
        .chunks(25)
        .map(|batch| {
            let mut card = Card::default();
            for y in 0..5 {
                for x in 0..5 {
                    card[y][x] = batch[y * 5 + x];
                }
            }
            BingoCard::new(card)
        })
        .collect();
    (balls, cards)
}

fn integer<'a, I: 'a>() -> impl Parser<I, Output = u8> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    repeat::many1(char::digit()).map(|string: String| string.parse::<u8>().unwrap())
}

fn balls<'a, I: 'a>() -> impl Parser<I, Output = Vec<u8>> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    repeat::sep_by(integer(), char::char(',')).skip(char::spaces())
}

fn cards_nums<'a, I: 'a>() -> impl Parser<I, Output = Vec<u8>> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    repeat::sep_by(integer(), char::spaces())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!((4512, 1924), run(input));
    }
}
