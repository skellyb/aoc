import {
  find_top_crates,
  find_top_multi_crates,
} from "../lib/bindings/bindings.ts";

export async function part1(input: string) {
  const cratesIds = find_top_crates(input);
  return cratesIds;
}

export async function part2(input: string) {
  const cratesIds = find_top_multi_crates(input);
  return cratesIds;
}
