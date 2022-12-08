import {
  find_top_crates,
  find_top_multi_crates,
} from "../lib/bindings/bindings.ts";

export function part1(input: string) {
  const cratesIds = find_top_crates(input);
  return cratesIds;
}

export function part2(input: string) {
  const cratesIds = find_top_multi_crates(input);
  return cratesIds;
}
