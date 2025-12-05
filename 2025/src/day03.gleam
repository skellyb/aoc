import gleam/int
import gleam/list
import gleam/string

pub fn run_pt1(input: String) -> Int {
  string.split(input, "\n")
  |> list.map(fn(row) { string.split(row, "") })
  |> list.map(fn(row) {
    list.fold(row, [], fn(accum, val_str) {
      let val = case int.parse(val_str) {
        Ok(i) -> i
        Error(_) -> panic as "couldn't parse string to int"
      }
      case accum {
        [] -> [val]
        [a] -> [a, val]
        [a, b] if b > a -> [b, val]
        [a, b] if val > b -> [a, val]
        _ -> accum
      }
    })
    |> list.fold(0, fn(accum, val) { accum * 10 + val })
  })
  |> list.fold(0, int.add)
}

pub fn run_pt2(input: String) -> Int {
  let size = 12
  parse(input)
  |> list.map(fn(row) { largest_subseq(row, size) })
  |> list.map(fn(row) { list.fold(row, 0, fn(accum, val) { accum * 10 + val }) })
  |> list.fold(0, int.add)
}

fn parse(input: String) -> List(List(Int)) {
  string.split(input, "\n")
  |> list.map(fn(row) {
    string.split(row, "")
    |> list.map(fn(char) {
      case int.parse(char) {
        Ok(i) -> i
        Error(_) -> panic as "couldn't parse string to int"
      }
    })
  })
}

fn largest_subseq(input: List(Int), size: Int) -> List(Int) {
  let #(stack, remain) = process(input, [], list.length(input) - size)
  case remain > 0 {
    True -> list.drop(stack, remain)
    False -> stack
  }
  |> list.reverse
}

fn process(queue: List(Int), stack: List(Int), remain: Int) -> #(List(Int), Int) {
  case queue {
    [head, ..rest] -> {
      let #(stack, rem) = subprocess(stack, head, remain)
      process(rest, [head, ..stack], rem)
    }
    [] -> #(stack, remain)
  }
}

fn subprocess(stack: List(Int), val: Int, remain: Int) -> #(List(Int), Int) {
  case stack {
    [head, ..rest] if remain > 0 && head < val -> {
      subprocess(rest, val, remain - 1)
    }
    _ -> #(stack, remain)
  }
}
