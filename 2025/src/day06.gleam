import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import glearray

pub fn run_pt1(input: String) -> Int {
  input
  |> string.split("\n")
  |> list.map(fn(row) { parse_line(row) })
  |> list.transpose
  |> list.group(fn(col) {
    case list.any(col, fn(input) { input == Operator(Sum) }) {
      True -> int.add
      False -> int.multiply
    }
  })
  |> dict.to_list
  |> list.map(fn(group) {
    let operator = group.0
    let lines = group.1
    list.fold(lines, 0, fn(acc, line) {
      line
      |> list.filter_map(fn(input) {
        case input {
          Operand(i) -> Ok(i)
          _ -> Error(Nil)
        }
      })
      |> list.reduce(fn(acc, input) { operator(acc, input) })
      |> result.unwrap(0)
      |> int.add(acc)
    })
  })
  |> list.fold(0, fn(acc, i) { acc + i })
}

pub fn run_pt2(input: String) -> Int {
  let rows = input |> string.split("\n")
  let op_row = case rows |> list.last {
    Ok(ops) -> string.to_graphemes(ops)
    Error(_) -> panic as "parse error"
  }
  let ops =
    op_row
    |> list.filter_map(fn(char) {
      case char {
        " " -> Error(Nil)
        "*" -> Ok(Multiply)
        "+" -> Ok(Sum)
        _ -> Error(Nil)
      }
    })
  let row_width = list.length(op_row)
  let chars =
    rows
    |> list.take(list.length(rows) - 1)
    |> list.map(fn(row) { string.to_graphemes(row) })
    |> list.flatten
  let refs = glearray.from_list(chars)
  list.range(0, row_width - 1)
  |> list.map(fn(i) { join_col(refs, row_width, 0, i, "") })
  |> group_inputs([], [])
  |> list.reverse
  |> list.zip(ops)
  |> list.map(fn(calc) {
    case calc.1 {
      Multiply -> list.fold(calc.0, 1, fn(acc, val) { acc * val })
      Sum -> int.sum(calc.0)
    }
  })
  |> int.sum
}

fn join_col(
  refs: glearray.Array(String),
  row_width: Int,
  row_idx: Int,
  col_idx: Int,
  str: String,
) -> String {
  let idx = col_idx + { row_width * row_idx }
  case glearray.get(refs, idx) {
    Error(_) -> str
    Ok(" ") -> join_col(refs, row_width, row_idx + 1, col_idx, str)
    Ok(char) -> {
      let update = string.append(str, char)
      join_col(refs, row_width, row_idx + 1, col_idx, update)
    }
  }
}

fn group_inputs(
  input: List(String),
  current: List(Int),
  accum: List(List(Int)),
) -> List(List(Int)) {
  case input {
    ["", ..rest] -> {
      group_inputs(rest, [], list.prepend(accum, current))
    }
    [str] -> {
      case int.parse(str) {
        Ok(i) ->
          group_inputs([], [], list.prepend(accum, list.prepend(current, i)))
        Error(_) -> panic as "parse error"
      }
    }
    [str, ..rest] -> {
      case int.parse(str) {
        Ok(i) -> group_inputs(rest, list.prepend(current, i), accum)
        Error(_) -> panic as "parse error"
      }
    }
    _ -> accum
  }
}

type Input {
  Operand(Int)
  Operator(Sign)
}

type Sign {
  Sum
  Multiply
}

fn parse_line(line: String) -> List(Input) {
  line
  |> string.split(" ")
  |> list.filter_map(fn(str) {
    case str {
      " " -> Error(Nil)
      "*" -> Ok(Operator(Multiply))
      "+" -> Ok(Operator(Sum))
      _ ->
        case int.parse(str) {
          Ok(i) -> Ok(Operand(i))
          _ -> Error(Nil)
        }
    }
  })
}
