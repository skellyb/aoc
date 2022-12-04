import { assertEquals } from "./deps.ts";
import { StringReader } from "./deps.ts";
import { part1, part2 } from "./day_03.ts";

const input = `vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
`;

Deno.test("d3:p1 test", async () => {
  assertEquals(await part1(new StringReader(input)), 157);
});

Deno.test("d3:p1 final", async () => {
  const file = await Deno.open("res/input_03.txt", { read: true });
  assertEquals(
    await part1(file),
    7821,
  );
  Deno.close(file.rid);
});

Deno.test("d3:p2 test", async () => {
  assertEquals(await part2(new StringReader(input)), 70);
});

Deno.test("d3:p2 final", async () => {
  const file = await Deno.open("res/input_03.txt", { read: true });
  assertEquals(
    await part2(file),
    2752,
  );
  Deno.close(file.rid);
});
