use combine::{choice, error::ParseError, parser::*, stream::RangeStream};

/// 1. Add up total horizontal movement and depth and multiply them together
/// 2. Down and up adjust 'aim' and forward changes depth using 'aim'
pub fn run(input: &str) -> (i32, i32) {
    let commands = parse_course(input);

    // part one
    let (horiz, depth) = commands.iter().fold((0, 0), |(h, d), cmd| match cmd {
        &Command::Forward(x) => (h + x, d),
        &Command::Up(x) => (h, d - x),
        &Command::Down(x) => (h, d + x),
    });

    // part two
    // down and up increase/decrease aim
    // fwd increases horiz by x AND increases depth by aim * x
    let (_, horiz2, depth2) = commands.iter().fold((0, 0, 0), |(a, h, d), cmd| match cmd {
        &Command::Forward(x) => (a, h + x, d + (a * x)),
        &Command::Up(x) => (a - x, h, d),
        &Command::Down(x) => (a + x, h, d),
    });

    (horiz * depth, horiz2 * depth2)
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_course(input: &str) -> Vec<Command> {
    let mut parser = repeat::many1(choice!(
        command("forward").map(|d| Command::Forward(d)),
        command("up").map(|d| Command::Up(d)),
        command("down").map(|d| Command::Down(d))
    ));
    let (commands, _) = parser.easy_parse(input).unwrap();
    commands
}

fn command<'a, I: 'a>(tag: &'a str) -> impl Parser<I, Output = i32> + 'a
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        range::range(tag).skip(char::spaces()),
        repeat::many1(char::digit()).skip(char::spaces()),
    )
        .map(|(_, num): (_, String)| num.parse().unwrap())
}
