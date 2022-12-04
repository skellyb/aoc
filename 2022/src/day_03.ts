import { readLines } from "./deps.ts";
import { type Reader } from "./deps.ts";

export async function part1(inputReader: Reader) {
  let total = 0;
  for await (const line of readLines(inputReader)) {
    const char = commonChar(split(line));
    if (char) {
      total += priority(char);
    }
  }
  return total;
}

export async function part2(inputReader: Reader) {
  let total = 0;
  let group = [];
  for await (const line of readLines(inputReader)) {
    group.push(line);
    if (group.length === 3) {
      const char = commonChar(group);
      if (char) {
        total += priority(char);
      }
      group = [];
    }
  }
  return total;
}

function split(line: string): [string, string] {
  return [line.slice(0, line.length / 2), line.slice(line.length / 2)];
}

function commonChar(lines: string[]) {
  const [first, ...rest] = lines;
  return [...first].find((c) => rest.every((ln) => ln.includes(c)));
}

const A_Z = "abcdefghijklmnopqrstuvwxyz";

function priority(char: string) {
  const normal = char.toLowerCase();
  const i = A_Z.indexOf(normal);
  return (char === normal ? i + 1 : i + 27);
}
