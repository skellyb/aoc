import day01
import day02
import day03
import gleam/int
import gleam/io
import gleam/string
import simplifile

pub fn main() -> Nil {
  io.println("AOC 2025")
  case simplifile.read("inputs/day_01.txt") {
    Ok(input) -> {
      io.println("Day 1")
      let instructions = string.split(input, "\n")
      let result = day01.run_pt1(instructions, 50, 0)
      io.println("Part 1: " <> int.to_string(result))
      let result2 = day01.run_pt2(instructions, 50, 0)
      io.println("Part 2: " <> int.to_string(result2))
    }
    Error(_) -> {
      io.println("error reading file")
    }
  }

  case simplifile.read("inputs/day_02.txt") {
    Ok(input) -> {
      io.println("Day 2")
      let result = day02.run_pt1(input)
      io.println("Part 1: " <> int.to_string(result))
      let result2 = day02.run_pt2(input)
      io.println("Part 2: " <> int.to_string(result2))
    }
    Error(_) -> {
      io.println("error reading file")
    }
  }

  case simplifile.read("inputs/day_03.txt") {
    Ok(input) -> {
      io.println("Day 3")
      let result = day03.run_pt1(input)
      io.println("Part 1: " <> int.to_string(result))
      let result2 = day03.run_pt2(input)
      io.println("Part 2: " <> int.to_string(result2))
    }
    Error(_) -> {
      io.println("error reading file")
    }
  }
}
