use std::collections::{HashMap, HashSet};

/// 1. count the number of 1, 4, 7, or 8s in the output codes
/// 2. decode outputs numbers
pub fn run(input: &str) -> (i32, i32) {
    let pt1 = {
        let notes = parse(input);
        let outputs: Vec<&str> = notes.iter().fold(vec![], |mut acc, n| {
            acc.extend(n.outputs.clone());
            acc
        });
        outputs.iter().fold(0, |acc, o| {
            acc + match o.len() {
                2 | 4 | 3 | 7 => 1,
                _ => 0,
            }
        })
    };
    let pt2 = {
        let notes = parse(input);
        notes.iter().fold(0, |acc, n| {
            let mut decoder = Decoder::new(&n.signals);
            let num_str: String = n
                .outputs
                .iter()
                .map(|code| decoder.apply(code).to_string())
                .collect();
            let num: i32 = num_str.parse().unwrap();
            acc + num
        })
    };
    (pt1, pt2)
}

#[derive(Debug)]
struct Note<'a> {
    signals: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

fn parse(input: &str) -> Vec<Note> {
    let ins_outs: Vec<Vec<&str>> = input.lines().map(|l| l.split(" | ").collect()).collect();
    ins_outs
        .iter()
        .map(|io| {
            let signals = io[0].split(' ').collect();
            let outputs = io[1].split(' ').collect();
            Note { signals, outputs }
        })
        .collect()
}

struct Decoder(HashMap<i32, HashSet<char>>);

impl Decoder {
    fn new(signals: &Vec<&str>) -> Self {
        let mut guide = Decoder(HashMap::new());
        for code in signals {
            match code.len() {
                2 => {
                    guide.0.insert(1 as i32, code.chars().collect());
                }
                3 => {
                    guide.0.insert(7 as i32, code.chars().collect());
                }
                4 => {
                    guide.0.insert(4 as i32, code.chars().collect());
                }
                7 => {
                    guide.0.insert(8 as i32, code.chars().collect());
                }
                _ => (),
            }
        }
        guide
    }

    fn apply(&mut self, code: &str) -> i32 {
        let comp_set: HashSet<char> = code.chars().collect();
        let one = self.0.get(&1).unwrap();
        let four = self.0.get(&4).unwrap();
        match code.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => match comp_set.difference(one).count() {
                3 => {
                    self.0.insert(3 as i32, code.chars().collect());
                    3
                }
                _ => match comp_set.difference(four).count() {
                    3 => {
                        self.0.insert(2 as i32, code.chars().collect());
                        2
                    }
                    _ => {
                        self.0.insert(5 as i32, code.chars().collect());
                        5
                    }
                },
            },
            6 => match comp_set.difference(one).count() {
                5 => {
                    self.0.insert(6 as i32, code.chars().collect());
                    6
                }
                _ => match comp_set.difference(four).count() {
                    3 => {
                        self.0.insert(0 as i32, code.chars().collect());
                        0
                    }
                    _ => {
                        self.0.insert(9 as i32, code.chars().collect());
                        9
                    }
                },
            },
            7 => 8,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!((26, 61229), run(input));
    }
}
