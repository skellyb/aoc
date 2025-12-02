import gleam/int
import gleam/string

pub fn run_pt1(instructions: List(String), position: Int, zeros: Int) -> Int {
  case instructions {
    [] -> zeros
    [next, ..rest] -> {
      let turn = parse(string.trim(next))
      let new_pos = wrap(position + turn)
      let new_zeros = case new_pos == 0 {
        True -> zeros + 1
        False -> zeros
      }
      run_pt1(rest, new_pos, new_zeros)
    }
  }
}

pub fn run_pt2(instructions: List(String), position: Int, zeros: Int) -> Int {
  case instructions {
    [] -> zeros
    [next, ..rest] -> {
      let delta = parse(string.trim(next))
      let #(pos, passes) = simulate(position, delta, zeros)
      run_pt2(rest, pos, passes)
    }
  }
}

fn parse(input: String) -> Int {
  let dir = string.slice(input, 0, 1)
  let dist_str = string.drop_start(input, 1)
  case int.parse(dist_str) {
    Ok(distance) ->
      case dir {
        "L" -> -distance
        "R" -> distance
        _ -> 0
      }
    Error(_) -> 0
  }
}

fn simulate(position: Int, move: Int, pass: Int) -> #(Int, Int) {
  case move {
    0 -> #(position, pass)
    i if i < 0 -> {
      let next_pos = wrap(position - 1)
      let next_pass = pass_zero_check(next_pos, pass)
      simulate(next_pos, move + 1, next_pass)
    }
    i if i > 0 -> {
      let next_pos = wrap(position + 1)
      let next_pass = pass_zero_check(next_pos, pass)
      simulate(next_pos, move - 1, next_pass)
    }
    _ -> #(position, pass)
  }
}

fn wrap(delta: Int) -> Int {
  let pos = delta % 100
  case pos < 0 {
    True -> pos + 100
    False -> pos
  }
}

fn pass_zero_check(pos: Int, pass: Int) -> Int {
  case pos {
    0 -> pass + 1
    _ -> pass
  }
}
