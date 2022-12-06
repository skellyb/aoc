import { assertEquals } from "./deps.ts";
import { part1, part2 } from "./day_05.ts";

const input = `    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
`;

Deno.test("d5:p1 test", async () => {
  assertEquals(await part1(input), "CMZ");
});

Deno.test("d5:p1 final", async () => {
  const txt = await Deno.readTextFile("res/input_05.txt");
  assertEquals(
    await part1(txt),
    "RFFFWBPNS",
  );
});

Deno.test("d5:p2 test", async () => {
  assertEquals(await part2(input), "MCD");
});

Deno.test("d5:p2 final", async () => {
  const txt = await Deno.readTextFile("res/input_05.txt");
  assertEquals(
    await part2(txt),
    "CQQBBJFCS",
  );
});
