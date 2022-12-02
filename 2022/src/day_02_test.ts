import { assertEquals } from "./deps.ts";
import { StringReader } from "./deps.ts";
import { part1, part2 } from "./day_02.ts";

const input = `A Y
B X
C Z
`;

Deno.test("d2:p1 test", async () => {
  assertEquals(await part1(new StringReader(input)), 15);
});

Deno.test("d2:p1 final", async () => {
  const file = await Deno.open("res/input_02.txt", { read: true });
  assertEquals(
    await part1(file),
    15422,
  );
  Deno.close(file.rid);
});

Deno.test("d2:p2 test", async () => {
  assertEquals(await part2(new StringReader(input)), 12);
});

Deno.test("d2:p2 final", async () => {
  const file = await Deno.open("res/input_02.txt", { read: true });
  assertEquals(
    await part2(file),
    15442,
  );
  Deno.close(file.rid);
});
