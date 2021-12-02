mod day_01;
mod day_02;

fn main() {
    let input = include_str!("../res/day-01.txt");
    let (d1p1, d1p2) = day_01::run(input);
    println!("Day 1: {} {}", d1p1, d1p2);

    let input = include_str!("../res/day-02.txt");
    let (d2p1, d2p2) = day_02::run(input);
    println!("Day 2: {} {}", d2p1, d2p2);
}
