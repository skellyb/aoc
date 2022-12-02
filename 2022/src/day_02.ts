import { readLines } from "./deps.ts";
import { type Reader } from "./deps.ts";

type Left = "A" | "B" | "C";
type Right = "X" | "Y" | "Z";
type Match = [Left, " ", Right];

// rock = A | X, paper = B | Y, scissors = C | Z
// win = 6, draw = 3, lose = 0
const matchPoints = {
  A: { Z: 0, X: 3, Y: 6 },
  B: { X: 0, Y: 3, Z: 6 },
  C: { Y: 0, Z: 3, X: 6 },
};

// rock = 1, paper = 2, rock = 3
const shapePoints = {
  X: 1,
  Y: 2,
  Z: 3,
};

export async function part1(inputReader: Reader) {
  let total = 0;
  for await (const line of readLines(inputReader)) {
    const [left, , right] = line as unknown as Match;
    total += matchPoints[left][right] + shapePoints[right];
  }
  return total;
}

// lose = X, draw = Y, win = Z
const outcomePoints = {
  X: 0,
  Y: 3,
  Z: 6,
};

// rock = 1, paper = 2, rock = 3
const outcomeShapePoints = {
  A: { X: 3, Y: 1, Z: 2 },
  B: { X: 1, Y: 2, Z: 3 },
  C: { X: 2, Y: 3, Z: 1 },
};

export async function part2(inputReader: Reader) {
  let total = 0;
  for await (const line of readLines(inputReader)) {
    const [left, , right] = line as unknown as Match;
    total += outcomePoints[right] + outcomeShapePoints[left][right];
  }
  return total;
}
