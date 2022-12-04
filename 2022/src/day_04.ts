import { readLines } from "./deps.ts";
import { type Reader } from "./deps.ts";

type Pair<T> = [T, T];

export async function part1(inputReader: Reader) {
  let total = 0;
  for await (const line of readLines(inputReader)) {
    const [left, right] = parse(line);
    const lContain = left[0] <= right[0] &&
      left[1] >= right[1];
    const rContain = right[0] <= left[0] &&
      right[1] >= left[1];
    if (lContain || rContain) {
      total += 1;
    }
  }
  return total;
}

export async function part2(inputReader: Reader) {
  let total = 0;
  for await (const line of readLines(inputReader)) {
    const [left, right] = parse(line);
    const first = left[0] < right[0] ? left : right;
    const second = left[0] < right[0] ? right : left;
    if (first[1] >= second[0]) {
      total += 1;
    }
  }
  return total;
}

function parse(line: string): Pair<Pair<number>> {
  const [left, right] = line.split(",");
  return [parsePair(left), parsePair(right)];
}

function parsePair(input: string): Pair<number> {
  const [left, right] = input.split("-");
  return [parseInt(left), parseInt(right)];
}
