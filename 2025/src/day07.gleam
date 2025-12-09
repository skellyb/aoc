import gleam/dict
import gleam/list
import gleam/string
import glearray

pub fn run_pt1(input: String) -> Int {
  let result =
    input
    |> string.split("\n")
    |> list.map(fn(row) { string.to_graphemes(row) })
    |> list.fold(#([], 0), fn(acc, row) { update_row(row, 0, acc.0, acc.1) })
  result.1
}

fn update_row(
  row: List(String),
  x: Int,
  beams: List(Int),
  hits: Int,
) -> #(List(Int), Int) {
  case row {
    ["S", ..] -> update_row([], 0, [x], hits)
    [] -> #(beams, hits)
    [_] -> update_row([], 0, beams, hits)
    [col, ..rest] -> {
      case col {
        "^" -> {
          case list.contains(beams, x) {
            True -> {
              let update =
                [x - 1, x + 1, ..beams]
                |> list.filter(fn(b) { b != x })
                |> list.unique
              update_row(rest, x + 1, update, hits + 1)
            }
            False -> update_row(rest, x + 1, beams, hits)
          }
        }
        _ -> update_row(rest, x + 1, beams, hits)
      }
    }
  }
}

pub fn run_pt2(input: String) -> Int {
  let x = parse_root(input)
  let #(count, _) = parse_edges(input) |> count_paths(#(x, 0), dict.new())
  count
}

type Coord =
  #(Int, Int)

pub type Edge {
  Edge(from: Coord, to: Coord)
}

fn parse_root(input: String) -> Int {
  case string.split_once(input, "S") {
    Ok(#(head, _)) -> string.length(head)
    Error(_) -> panic as "parse error"
  }
}

fn count_paths(
  edges: List(Edge),
  current: Coord,
  cache: dict.Dict(Coord, Int),
) -> #(Int, dict.Dict(Coord, Int)) {
  case dict.get(cache, current) {
    Ok(count) -> #(count, cache)
    Error(_) -> {
      let outgoing = list.filter(edges, fn(e) { e.from == current })
      case outgoing {
        [] -> {
          let new_cache = dict.insert(cache, current, 1)
          #(1, new_cache)
        }
        children -> {
          let #(total, final_cache) =
            list.fold(children, #(0, cache), fn(acc, e) {
              let #(sum, curr_cache) = acc
              let #(child_count, updated_cache) =
                count_paths(edges, e.to, curr_cache)
              #(sum + child_count, updated_cache)
            })
          let new_cache = dict.insert(final_cache, current, total)
          #(total, new_cache)
        }
      }
    }
  }
}

fn parse_edges(input: String) {
  let rows =
    input
    |> string.split("\n")
    |> list.map(fn(row) { string.to_graphemes(row) })
  let refs = glearray.from_list(list.flatten(rows))
  let row_width = case list.first(rows) {
    Ok(r) -> list.length(r)
    Error(_) -> panic as "parse error"
  }
  glearray.to_list(refs)
  |> list.index_fold([], fn(acc, char, idx) {
    let x = idx % row_width
    let y = idx / row_width
    case char {
      "S" -> [Edge(#(x, y), #(x, beam(refs, x, y + 1, row_width)))]
      "^" -> [
        Edge(#(x, y), #(x - 1, beam(refs, x - 1, y + 1, row_width))),
        Edge(#(x, y), #(x + 1, beam(refs, x + 1, y + 1, row_width))),
        ..acc
      ]
      _ -> acc
    }
  })
}

fn beam(refs: glearray.Array(String), x: Int, y: Int, width: Int) -> Int {
  let idx = y * width + x
  case glearray.get(refs, idx) {
    Ok("^") -> y
    Ok(_) -> beam(refs, x, y + 1, width)
    Error(_) -> y
  }
}
