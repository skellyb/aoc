use std::collections::HashSet;

use combine::{
    choice,
    easy::{Error::Unexpected, Info},
    error::ParseError,
    parser,
    parser::*,
    stream::RangeStream,
};

// 1. Discard incomplete lines and find corrupted characters
// 2. Discard corrupted lines and complete incomplete lines
pub fn run(input: &str) -> (u64, u64) {
    let pt1 = {
        input
            .lines()
            .filter_map(|l| match parse(l) {
                Ok(_) => None,
                Err(err) => match err {
                    LineError::BadChar(c) => Some(map_to_pts(c)),
                    _ => None,
                },
            })
            .sum()
    };

    let pt2 = {
        let incomplete: Vec<&str> = input
            .lines()
            .filter(|l| match parse(l) {
                Ok(_) => false,
                Err(err) => match err {
                    LineError::Incomplete => true,
                    _ => false,
                },
            })
            .collect();
        let mut pts: Vec<u64> = incomplete
            .iter()
            .map(|s| {
                completions(s)
                    .iter()
                    .fold(0, |acc, c| acc * 5 + map_to_pts_2(c))
            })
            .collect();
        pts.sort();
        pts[pts.len() / 2]
    };
    (pt1, pt2)
}

fn map_to_pts(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn map_to_pts_2(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn completions(input: &str) -> Vec<char> {
    let open_set: HashSet<char> = HashSet::from_iter(['(', '[', '{', '<']);
    let close_set: HashSet<char> = HashSet::from_iter([')', ']', '}', '>']);
    let mut queue: Vec<char> = vec![];
    for c in input.chars() {
        match (open_set.contains(&c), close_set.contains(&c)) {
            (true, false) => queue.push(c),
            (false, true) => {
                queue.pop();
            }
            _ => (),
        }
    }
    queue
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("invalid character"),
        })
        .collect()
}

#[derive(Debug)]
enum LineError {
    Incomplete,
    BadChar(char),
    Unknown,
}

fn parse(line: &str) -> Result<(), LineError> {
    match repeat::many1(chunk()).map(|_: Vec<()>| ()).easy_parse(line) {
        Ok(_) => Ok(()),
        Err(err) => {
            let kind = err.errors.iter().next().unwrap();
            match kind {
                Unexpected(Info::Token(t)) => Err(LineError::BadChar(*t)),
                Unexpected(Info::Static("end of input")) => Err(LineError::Incomplete),
                _ => Err(LineError::Unknown),
            }
        }
    }
}

fn _chunk<'a, I: 'a>() -> impl Parser<I, Output = ()> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice!(
        sequence::between(
            char::char('('),
            char::char(')'),
            repeat::many(chunk()).map(|_: Vec<()>| ())
        ),
        sequence::between(
            char::char('['),
            char::char(']'),
            repeat::many(chunk()).map(|_: Vec<()>| ())
        ),
        sequence::between(
            char::char('{'),
            char::char('}'),
            repeat::many(chunk()).map(|_: Vec<()>| ())
        ),
        sequence::between(
            char::char('<'),
            char::char('>'),
            repeat::many(chunk()).map(|_: Vec<()>| ())
        )
    )
}

// This macro makes recursive parsing possible
parser! {
    fn chunk['a, Input]()(Input) -> ()
    where [Input: RangeStream<Token = char, Range = &'a str> + 'a]
    {
        _chunk()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!((26397, 288957), run(input));
    }
}
