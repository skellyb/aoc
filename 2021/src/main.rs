mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    let input = include_str!("../res/day-01.txt");
    let (d1p1, d1p2) = day_01::run(input);
    println!("Day 1: {} {}", d1p1, d1p2);

    let input = include_str!("../res/day-02.txt");
    let (d2p1, d2p2) = day_02::run(input);
    println!("Day 2: {} {}", d2p1, d2p2);

    let input = include_str!("../res/day-03.txt");
    let (d3p1, d3p2) = day_03::run(input);
    println!("Day 3: {} {}", d3p1, d3p2);

    let input = include_str!("../res/day-04.txt");
    let (d4p1, d4p2) = day_04::run(input);
    println!("Day 4: {} {}", d4p1, d4p2);

    let input = include_str!("../res/day-05.txt");
    let (d5p1, d5p2) = day_05::run(input);
    println!("Day 5: {} {}", d5p1, d5p2);

    let input = include_str!("../res/day-06.txt");
    let (d6p1, d6p2) = day_06::run(input);
    println!("Day 6: {} {}", d6p1, d6p2);

    let input = include_str!("../res/day-07.txt");
    let (d7p1, d7p2) = day_07::run(input);
    println!("Day 7: {} {}", d7p1, d7p2);
}
