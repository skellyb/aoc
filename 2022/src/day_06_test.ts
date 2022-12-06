import { assertEquals } from "./deps.ts";
import { part1, part2 } from "./day_06.ts";

Deno.test("d6:p1 test", () => {
  assertEquals(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
  assertEquals(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
  assertEquals(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
  assertEquals(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
});

Deno.test("d6:p1 final", async () => {
  const txt = await Deno.readTextFile("res/input_06.txt");
  assertEquals(
    part1(txt),
    1876,
  );
});

Deno.test("d6:p2 test", () => {
  assertEquals(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
  assertEquals(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
  assertEquals(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
  assertEquals(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
  assertEquals(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
});

Deno.test("d6:p2 final", async () => {
  const txt = await Deno.readTextFile("res/input_06.txt");
  assertEquals(
    part2(txt),
    2202,
  );
});
