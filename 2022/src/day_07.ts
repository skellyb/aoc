import { disk_free, disk_usage } from "../lib/bindings/bindings.ts";

export function part1(input: string) {
  return disk_usage(input);
}

export function part2(input: string) {
  return disk_free(input);
}
