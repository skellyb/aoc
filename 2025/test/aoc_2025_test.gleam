import day01
import day02
import day03
import gleam/string
import gleeunit

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn day1_pt1_test() {
  let input =
    "L68
  L30
  R48
  L5
  R60
  L55
  L1
  L99
  R14
  L82"
  let instructions = string.split(input, "\n")
  assert day01.run_pt1(instructions, 50, 0) == 3
}

pub fn day1_pt2_test() {
  let input =
    "L68
  L30
  R48
  L5
  R60
  L55
  L1
  L99
  R14
  L82"
  let instructions = string.split(input, "\n")
  assert day01.run_pt2(instructions, 50, 0) == 6
}

pub fn day2_pt1_test() {
  let input =
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
  assert day02.run_pt1(input) == 1_227_775_554
}

pub fn day2_pt2_test() {
  let input =
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
  assert day02.run_pt2(input) == 4_174_379_265
}

pub fn day3_pt1_test() {
  let input =
    "987654321111111
811111111111119
234234234234278
818181911112111"
  assert day03.run_pt1(input) == 357
}

pub fn day3_pt2_test() {
  let input =
    "987654321111111
811111111111119
234234234234278
818181911112111"
  assert day03.run_pt2(input) == 3_121_910_778_619
}
