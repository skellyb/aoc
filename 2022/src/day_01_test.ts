import { assertEquals } from "./deps.ts";
import { StringReader } from "./deps.ts";
import { part1, part2 } from "./day_01.ts";

const input = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
`;

Deno.test("d1:p1 test", async () => {
  assertEquals(await part1(new StringReader(input)), 24000);
});

Deno.test("d1:p1 final", async () => {
  const file = await Deno.open("res/input_01.txt", { read: true });
  assertEquals(
    await part1(file),
    66306,
  );
  Deno.close(file.rid);
});

Deno.test("d1:p2 test", async () => {
  assertEquals(await part2(new StringReader(input)), 45000);
});

Deno.test("d1:p2 final", async () => {
  const file = await Deno.open("res/input_01.txt", { read: true });
  assertEquals(
    await part2(file),
    195292,
  );
  Deno.close(file.rid);
});
