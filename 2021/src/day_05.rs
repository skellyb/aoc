use combine::{error::ParseError, parser::*, stream::RangeStream};

const GRID_SIZE: usize = 1000;

/// 1. Find the overlapping points (only horizontal and vertical lines)
/// 2. Find the overlapping points (only horizontal and vertical and 45 deg lines)
pub fn run(input: &str) -> (u32, u32) {
    // part one
    let pt1 = {
        let segments = parse(input);
        let horiz_vert: Vec<&Segment> = segments
            .iter()
            .filter(|s| s.start.x == s.end.x || s.start.y == s.end.y)
            .collect();
        let mut grid: Grid = Vec::new();
        grid.resize(GRID_SIZE, vec![0; GRID_SIZE]);
        for seg in horiz_vert {
            mark(&mut grid, seg);
        }
        hits(&grid, 1)
    };

    // part two
    let pt2 = {
        let segments = parse(input);
        let horiz_vert_diag: Vec<&Segment> = segments
            .iter()
            .filter(|s| {
                (s.start.x == s.end.x || s.start.y == s.end.y)
                    || (i32::abs(s.end.x - s.start.x) == i32::abs(s.end.y - s.start.y))
            })
            .collect();
        let mut grid: Grid = Vec::new();
        grid.resize(GRID_SIZE, vec![0; GRID_SIZE]);
        for seg in horiz_vert_diag {
            mark(&mut grid, seg);
        }
        hits(&grid, 1)
    };

    (pt1, pt2)
}

fn mark(grid: &mut Grid, segment: &Segment) {
    let end_x = segment.end.x;
    let end_y = segment.end.y;
    let mut x = segment.start.x;
    let mut y = segment.start.y;
    loop {
        grid[y as usize][x as usize] += 1;
        if x == end_x && y == end_y {
            break;
        }
        if x < end_x {
            x += 1;
        } else if x > end_x {
            x -= 1;
        }
        if y < end_y {
            y += 1;
        } else if y > end_y {
            y -= 1;
        }
    }
}

fn hits(grid: &Grid, threshold: u32) -> u32 {
    let mut count = 0;
    for row in grid {
        for n in row {
            if *n > threshold {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn show(grid: &Grid) {
    for row in grid {
        for n in row {
            if *n == 0 {
                print!(".");
            } else {
                print!("{}", n);
            }
        }
        print!("\n");
    }
}

type Grid = Vec<Vec<u32>>;

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> Vec<Segment> {
    let (segments, _) = repeat::sep_by1(segment(), char::newline())
        .easy_parse(input)
        .unwrap();
    segments
}

fn integer<'a, I: 'a>() -> impl Parser<I, Output = i32> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    repeat::many1(char::digit()).map(|string: String| string.parse::<i32>().unwrap())
}

fn coord<'a, I: 'a>() -> impl Parser<I, Output = Point> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (integer().skip(char::char(',')), integer()).map(|(x, y)| Point { x, y })
}

fn segment<'a, I: 'a>() -> impl Parser<I, Output = Segment> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (coord().skip(range::range(" -> ")), coord()).map(|(p1, p2)| Segment { start: p1, end: p2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!((5, 12), run(input));
    }
}
