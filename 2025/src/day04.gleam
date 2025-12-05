import gleam/list
import gleam/string
import glearray

pub fn run_pt1(input: String) -> Int {
  let #(positions, _) = parse(input)
  positions
  |> list.filter(fn(pos) { pos == Open })
  |> list.length
}

pub fn run_pt2(input: String) -> Int {
  let #(positions, row_size) = parse(input)
  let occupied_start =
    list.filter(positions, fn(pos) { pos != Empty }) |> list.length
  let end_positions = remove_opens(positions, row_size)
  let occupied_finish =
    list.filter(end_positions, fn(pos) { pos != Empty }) |> list.length
  occupied_start - occupied_finish
}

pub type Position {
  Empty
  Blocked
  Open
}

fn parse(input: String) -> #(List(Position), Int) {
  let symbol_grid =
    input
    |> string.split("\n")
    |> list.map(fn(str) { string.to_graphemes(str) })
  let row_size = case list.first(symbol_grid) {
    Ok(rows) -> list.length(rows)
    Error(_) -> panic as "parse error: no rows"
  }
  let positions =
    list.flatten(symbol_grid)
    |> list.map(fn(symbol) {
      case symbol {
        "." -> Empty
        "@" -> Blocked
        _ -> panic as "parse error: invalid symbol"
      }
    })
  #(update(positions, row_size), row_size)
}

fn remove_opens(positions: List(Position), row_size: Int) -> List(Position) {
  let curr_empty =
    list.filter(positions, fn(pos) { pos == Empty }) |> list.length
  let removed =
    list.map(positions, fn(pos) {
      case pos {
        Open -> Empty
        _ -> pos
      }
    })
  let now_empty = list.filter(removed, fn(pos) { pos == Empty }) |> list.length
  case now_empty - curr_empty {
    0 -> removed
    _ -> remove_opens(update(removed, row_size), row_size)
  }
}

fn update(positions: List(Position), row_size: Int) -> List(Position) {
  let pos_refs = glearray.from_list(positions)
  list.index_map(positions, fn(pos, idx) {
    case pos {
      Empty -> Empty
      Open | Blocked -> {
        case check_blocked(idx, row_size, pos_refs) {
          True -> Blocked
          False -> Open
        }
      }
    }
  })
}

fn check_blocked(
  idx: Int,
  row_size: Int,
  refs: glearray.Array(Position),
) -> Bool {
  let col_idx = idx % row_size
  let horizontal = case col_idx {
    0 -> [idx + 1]
    c if c == row_size - 1 -> [idx - 1]
    _ -> [idx - 1, idx + 1]
  }
  let vertical = [
    idx - row_size,
    idx + row_size,
  ]
  let diagonals = case col_idx {
    0 -> [idx - row_size + 1, idx + row_size + 1]
    c if c == row_size - 1 -> [idx - row_size - 1, idx + row_size - 1]
    _ -> [
      idx - row_size - 1,
      idx - row_size + 1,
      idx + row_size - 1,
      idx + row_size + 1,
    ]
  }
  let checks = list.flatten([horizontal, vertical, diagonals])
  let occupied_count =
    checks
    |> list.filter(fn(i) {
      case col_idx == 0 {
        True -> i != idx - 1
        False ->
          case col_idx == row_size - 1 {
            True -> i != idx + 1
            False -> True
          }
      }
    })
    |> list.filter(fn(i) {
      case glearray.get(refs, i) {
        Ok(pos) ->
          case pos {
            Empty -> False
            Blocked | Open -> True
          }
        Error(_) -> False
      }
    })
    |> list.length
  occupied_count > 3
}
