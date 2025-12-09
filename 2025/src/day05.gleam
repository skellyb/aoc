import gleam/int
import gleam/list
import gleam/string

pub fn run_pt1(input: String) -> Int {
  parse(input)
  |> list.filter(fn(item) {
    case item {
      Fresh(_) -> True
      _ -> False
    }
  })
  |> list.length
}

pub fn run_pt2(input: String) -> Int {
  let #(_, total) = case string.split(input, "\n\n") {
    [ranges, _] -> {
      parse_ranges(ranges)
      |> list.map(fn(r) {
        case r {
          [a, b] -> #(a, b)
          _ -> panic as "parse error"
        }
      })
      |> list.sort(fn(a, b) { int.compare(a.0, b.0) })
      |> list.fold(#(#(0, 0), 0), fn(acc, val) {
        case acc {
          #(#(a, b), total) if val.0 <= b && val.1 > b -> #(
            #(a, val.1),
            total + val.1 - b,
          )
          #(#(a, b), total) if val.0 <= b && val.1 <= b -> #(#(a, b), total)
          #(_, total) -> #(val, total + val.1 - val.0 + 1)
        }
      })
    }
    _ -> panic as "parse error"
  }
  total
}

type Item {
  Fresh(Int)
  Spoiled(Int)
}

fn parse(input: String) {
  case string.split(input, "\n\n") {
    [ranges, ids] -> {
      let ranges = parse_ranges(ranges)
      let ids = parse_ids(ids)
      ids
      |> list.map(fn(id) {
        case ranges_contain(ranges, id) {
          True -> Fresh(id)
          False -> Spoiled(id)
        }
      })
    }
    _ -> panic as "parse error"
  }
}

fn ranges_contain(ranges: List(List(Int)), id: Int) -> Bool {
  let contains =
    list.filter(ranges, fn(r) {
      case r {
        [a, b] if id >= a && id <= b -> {
          True
        }
        _ -> False
      }
    })
    |> list.length
  contains > 0
}

fn parse_ranges(input: String) -> List(List(Int)) {
  string.split(input, "\n")
  |> list.map(fn(line) {
    string.split(line, "-")
    |> list.map(fn(digit) {
      case int.parse(digit) {
        Ok(d) -> d
        Error(_) -> panic as "parse error"
      }
    })
  })
}

fn parse_ids(input: String) -> List(Int) {
  string.split(input, "\n")
  |> list.map(fn(digit) {
    case int.parse(digit) {
      Ok(d) -> d
      Error(_) -> panic as "parse error"
    }
  })
}
