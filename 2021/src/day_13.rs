use combine::{error::ParseError, parser::*, stream::RangeStream};

// 1. Points visible after the first fold
// 2. Complete folds and output result
pub fn run(input: &str) -> (i32, i32) {
    let pt1 = {
        let (points, folds) = parse(input);
        let folded = fold_on(&points, &folds[0]);
        folded.len() as i32
    };

    let pt2 = {
        let (mut points, folds) = parse(input);
        for f in folds {
            points = fold_on(&points, &f);
        }
        display(&points);
        0
    };
    (pt1, pt2)
}

fn fold_on(points: &Vec<Point>, axis: &Fold) -> Vec<Point> {
    let mut folded: Vec<Point> = points
        .iter()
        .map(|p| match axis {
            Fold::X(x) if p.x > *x => {
                let flipped_x = (p.x - x) * -1 + x;
                Point {
                    x: flipped_x,
                    y: p.y,
                }
            }
            Fold::Y(y) if p.y > *y => {
                let flipped_y = (p.y - y) * -1 + y;
                Point {
                    x: p.x,
                    y: flipped_y,
                }
            }
            _ => Point { x: p.x, y: p.y },
        })
        .collect();
    folded.sort();
    folded.dedup();
    folded
}

fn display(points: &Vec<Point>) {
    for y in 0..8 {
        for x in 0..40 {
            if points.iter().find(|p| p.x == x && p.y == y).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn parse(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let ((mut points, folds), _): ((Vec<Point>, Vec<Fold>), _) = (
        repeat::many1(coord().skip(char::newline())),
        char::newline(),
        repeat::many1(fold().skip(choice::optional(char::newline()))),
    )
        .map(|(p, _, f)| (p, f))
        .easy_parse(input)
        .unwrap();
    points.sort();
    (points, folds)
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

fn fold<'a, I: 'a>() -> impl Parser<I, Output = Fold> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        range::range("fold along").skip(char::spaces()),
        choice::or(char::char('x'), char::char('y')).skip(char::char('=')),
        integer(),
    )
        .map(|(_, xy, int)| match xy {
            'x' => Fold::X(int),
            'y' => Fold::Y(int),
            _ => panic!("bad fold"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!((17, 0), run(input));
    }
}
