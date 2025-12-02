import gleam/int
import gleam/list
import gleam/string

pub fn run_pt1(input: String) -> Int {
  ranges(input)
  |> list.map(fn(pair) {
    let #(start, end) = case pair {
      [s, e] -> #(s, e)
      _ -> panic as "bad pair"
    }
    find_ids_in_range(start, end)
  })
  |> list.flatten
  |> list.fold(0, int.add)
}

pub fn run_pt2(input: String) -> Int {
  ranges(input)
  |> list.map(fn(pair) {
    let #(start, end) = case pair {
      [s, e] -> #(s, e)
      _ -> panic as "bad pair"
    }
    find_pattern_ids_in_range(start, end)
  })
  |> list.flatten
  |> list.fold(0, int.add)
}

fn ranges(input: String) -> List(List(Int)) {
  string.split(input, ",")
  |> list.map(fn(str) {
    string.trim(str)
    |> string.split("-")
    |> list.map(fn(int_str) {
      case int.parse(int_str) {
        Ok(i) -> i
        Error(_) -> panic as "couldn't parse string to int"
      }
    })
  })
}

fn find_ids_in_range(start: Int, end: Int) -> List(Int) {
  list.range(start, end)
  |> list.filter(fn(i) {
    let str = int.to_string(i)
    let len = string.length(str)
    let half = len / 2
    case len % 2 == 0 {
      False -> False
      True -> {
        string.slice(str, 0, half) == string.slice(str, half, half)
      }
    }
  })
}

fn find_pattern_ids_in_range(start: Int, end: Int) -> List(Int) {
  list.range(start, end)
  |> list.filter(fn(i) {
    let str = int.to_string(i)
    let len = string.length(str)
    case len > 1 {
      False -> False
      True -> {
        list.range(1, len / 2)
        |> list.any(fn(pattern_size) {
          case len % pattern_size == 0 {
            False -> False
            True -> {
              let pattern = string.slice(str, 0, pattern_size)
              let compare = string.repeat(pattern, len / pattern_size)
              compare == str
            }
          }
        })
      }
    }
  })
}
