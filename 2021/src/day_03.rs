/// 1. Get total 0s and 1s for each bit and generate values for gamma and epsilon
/// 2. Filter down to single number bit by bit
pub fn run(input: &str) -> (u32, u32) {
    let rows: Vec<&str> = input.lines().collect();

    // part one
    let counts: Vec<(u32, u32)> = input
        .chars()
        .take_while(|c| *c != '\n')
        .enumerate()
        .map(|(i, _)| count_column(&rows, i))
        .collect();
    let gamma = gen_rating(&counts, |zeros, ones| if zeros > ones { '0' } else { '1' });
    let epsilon = gen_rating(&counts, |zeros, ones| if zeros < ones { '0' } else { '1' });

    // part two
    let o2 = filter_rating(&rows, |zeros, ones| if zeros > ones { '0' } else { '1' });
    let co2 = filter_rating(&rows, |zeros, ones| if zeros <= ones { '0' } else { '1' });

    (gamma * epsilon, o2 * co2)
}

fn count_column(rows: &Vec<&str>, col: usize) -> (u32, u32) {
    let mut zeros = 0;
    let mut ones = 0;
    for row in rows {
        match row.chars().nth(col).unwrap() {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => (),
        }
    }
    (zeros, ones)
}

fn gen_rating<F>(totals: &Vec<(u32, u32)>, f: F) -> u32
where
    F: Fn(u32, u32) -> char,
{
    let rating_str = totals.iter().fold(String::default(), |mut bit_str, count| {
        bit_str.push(f(count.0, count.1));
        bit_str
    });
    u32::from_str_radix(&rating_str, 2).unwrap()
}

fn filter_rating<F>(rows: &Vec<&str>, f: F) -> u32
where
    F: Fn(u32, u32) -> char,
{
    let mut col = 0;
    let mut filtered = rows.clone();
    while filtered.len() > 1 {
        let counts = count_column(&filtered, col);
        filtered = filtered
            .iter()
            .filter_map(|&row| {
                let pick = f(counts.0, counts.1);
                if let Some(c) = row.chars().nth(col) {
                    if c == pick {
                        Some(row)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        col += 1;
    }
    u32::from_str_radix(filtered[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(run(input), (198, 230));
    }
}
