import { readLines } from "./deps.ts";
import { type Reader } from "./deps.ts";

export async function part1(inputReader: Reader) {
  let cals = [0];
  for await (const line of readLines(inputReader)) {
    cals = addGroupTotal(line, cals);
  }
  cals.sort((a, b) => b - a);
  return cals[0];
}

export async function part2(inputReader: Reader) {
  let cals = [0];
  for await (const line of readLines(inputReader)) {
    cals = addGroupTotal(line, cals);
  }
  cals.sort((a, b) => b - a);
  return cals.slice(0, 3).reduce((accum, num) => accum + num, 0);
}

function addGroupTotal(part: string, groups: number[]) {
  if (part.length > 0) {
    let group = groups.pop();
    if (group != null) {
      group += parseInt(part);
      groups.push(group);
    }
  } else {
    groups.push(0);
  }
  return groups;
}
