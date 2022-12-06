use deno_bindgen::deno_bindgen;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{eof, map},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

#[deno_bindgen]
pub fn find_top_crates(input: &str) -> String {
    let plan = parse(input);
    let rearranged = plan.apply_moves();
    pick_tops(&rearranged)
}

#[deno_bindgen]
pub fn find_top_multi_crates(input: &str) -> String {
    let plan = parse(input);
    let rearranged = plan.apply_multi_moves();
    pick_tops(&rearranged)
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

type Stacks = Vec<Vec<String>>;

#[derive(Debug)]
struct Plan {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl Plan {
    fn new() -> Self {
        Plan {
            stacks: vec![],
            moves: vec![],
        }
    }

    /// Take stacks and moves and return new rearraged stacks
    fn apply_moves(&self) -> Stacks {
        let mut output = self.stacks.clone();
        for mv in self.moves.iter().rev() {
            for _ in 0..mv.count {
                let item = output[mv.from].pop().unwrap();
                output[mv.to].push(item);
            }
        }
        output
    }

    /// Save as apply moves, but each move is one action
    fn apply_multi_moves(&self) -> Stacks {
        let mut output = self.stacks.clone();
        for mv in self.moves.iter().rev() {
            let at = output[mv.from].len() - mv.count;
            let mut batch = output[mv.from].split_off(at);
            output[mv.to].append(&mut batch);
        }
        output
    }
}

fn parse(input: &str) -> Plan {
    // Reverse lines, parsing from bottom to top, making stacks easier to deal with
    // They all fold into a Plan
    input.lines().rev().fold(Plan::new(), |mut plan, ln| {
        // Try to parse move, then stack row, pushing results into the Plan
        match parse_move(ln) {
            Ok((_, mv)) => plan.moves.push(mv),
            Err(_) => {
                if ln.len() > 0 {
                    match parse_stacks_row(ln) {
                        Ok((_, row)) => {
                            for (i, val) in row.iter().enumerate() {
                                if i >= plan.stacks.len() {
                                    plan.stacks.push(vec![]);
                                }
                                if let Some(c) = val {
                                    plan.stacks[i].push(c.clone());
                                }
                            }
                        }
                        Err(error) => {
                            panic!("Error parsing input: {}", error);
                        }
                    }
                }
            }
        }
        plan
    })
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        preceded(
            tag("move "),
            tuple((digit1, tag(" from "), digit1, tag(" to "), digit1)),
        ),
        |(count, _, from, _, to): (&str, _, &str, _, &str)| Move {
            count: count.parse::<usize>().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        },
    )
    .parse(input)
}

fn parse_stacks_row(input: &str) -> IResult<&str, Vec<Option<String>>> {
    many0(map(
        terminated(tuple((anychar, anychar, anychar)), alt((tag(" "), eof))),
        |(_, c, _)| {
            if c.is_alphabetic() {
                Some(c.to_string())
            } else {
                None
            }
        },
    ))
    .parse(input)
}

fn pick_tops(stacks: &Stacks) -> String {
    let mut output = String::new();
    for s in stacks.iter() {
        output = output + s.last().unwrap();
    }
    output
}
