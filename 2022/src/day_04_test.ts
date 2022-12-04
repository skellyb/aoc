import { assertEquals } from "./deps.ts";
import { StringReader } from "./deps.ts";
import { part1, part2 } from "./day_04.ts";

const input = `2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
`;

Deno.test("d4:p1 test", async () => {
  assertEquals(await part1(new StringReader(input)), 2);
});

Deno.test("d4:p1 final", async () => {
  const file = await Deno.open("res/input_04.txt", { read: true });
  assertEquals(
    await part1(file),
    448,
  );
  Deno.close(file.rid);
});

Deno.test("d4:p2 test", async () => {
  assertEquals(await part2(new StringReader(input)), 4);
});

Deno.test("d4:p2 final", async () => {
  const file = await Deno.open("res/input_04.txt", { read: true });
  assertEquals(
    await part2(file),
    794,
  );
  Deno.close(file.rid);
});
