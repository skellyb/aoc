use combine::{error::ParseError, parser::*, stream::RangeStream};
use std::collections::HashMap;

// 1. Subtract least common from most common letter
// 2. Expand to 40 steps
pub fn run(input: &str) -> (i64, i64) {
    let pt1 = {
        let (starter, rules) = parse(input);
        let mut result = starter;
        for _ in 0..10 {
            result = step(&result, &rules);
        }
        let mut t: Vec<i64> = totals(&result).into_iter().map(|(_, v)| v).collect();
        t.sort();
        t.last().unwrap() - t.first().unwrap()
    };

    let pt2 = {
        let (starter, rules) = parse(input);
        let mut total_chars = HashMap::new();
        for c in starter.iter() {
            total_chars.insert(
                *c,
                if let Some(count) = total_chars.get(c) {
                    count + 1
                } else {
                    1
                },
            );
        }
        let first: PairCount = starter
            .windows(2)
            .map(|win| ([win[0], win[1]], 1))
            .collect();
        count(first, &mut total_chars, &rules, 1, 40);
        let mut t: Vec<i64> = total_chars.values().map(|&v| v).collect();
        t.sort();
        t.last().unwrap() - t.first().unwrap()
    };
    (pt1, pt2)
}

type Pair = [char; 2];
type Rules = HashMap<Pair, char>;
type PairCount = HashMap<Pair, i64>;
type CharCount = HashMap<char, i64>;

fn count(batch: PairCount, totals: &mut CharCount, rules: &Rules, step: usize, end: usize) {
    let mut next = HashMap::new();
    for (k, v) in batch.iter() {
        let insert = rules.get(k).unwrap();
        let p1 = [k[0], *insert];
        let p2 = [*insert, k[1]];
        let next_val_1 = if let Some(exist) = next.get(&p1) {
            *exist + *v
        } else {
            *v
        };
        let next_val_2 = if let Some(exist) = next.get(&p2) {
            *exist + *v
        } else {
            *v
        };
        next.insert(p1, next_val_1);
        next.insert(p2, next_val_2);
        totals.insert(
            *insert,
            if let Some(exist) = totals.get(insert) {
                *exist + *v
            } else {
                *v
            },
        );
    }
    if end > step {
        count(next, totals, rules, step + 1, end);
    }
}

fn step(chars: &Vec<char>, rules: &Rules) -> Vec<char> {
    chars
        .windows(2)
        .enumerate()
        .flat_map(|(i, cc)| {
            let insert = rules.get(&[cc[0], cc[1]]).unwrap();
            if i < chars.len() - 2 {
                vec![cc[0], *insert]
            } else {
                vec![cc[0], *insert, cc[1]]
            }
        })
        .collect()
}

fn totals(chars: &Vec<char>) -> HashMap<char, i64> {
    let mut t: HashMap<char, i64> = HashMap::new();
    for c in chars {
        t.insert(*c, if let Some(v) = t.get(c) { *v + 1 } else { 1 });
    }
    t
}

fn parse(input: &str) -> (Vec<char>, Rules) {
    let (result, _): ((Vec<char>, Vec<((char, char), char)>), &str) = (
        starter(),
        repeat::many1((
            rule_pair().skip(range::range(" -> ")),
            char::letter().skip(char::spaces()),
        )),
    )
        .easy_parse(input)
        .unwrap();
    let (start, rule_parts) = result;
    let rules: Rules = rule_parts
        .into_iter()
        .map(|(rp, c)| ([rp.0, rp.1], c))
        .collect();
    (start, rules)
}

fn starter<'a, I: 'a>() -> impl Parser<I, Output = Vec<char>> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    repeat::many1(char::letter()).skip(char::spaces())
}

fn rule_pair<'a, I: 'a>() -> impl Parser<I, Output = (char, char)> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (char::letter(), char::letter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!((1588, 2188189693529), run(input));
    }
}
