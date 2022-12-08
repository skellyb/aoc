import { assertEquals } from "./deps.ts";
import { part1, part2 } from "./day_07.ts";

const input = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
`;

Deno.test("d7:p1 test", () => {
  assertEquals(part1(input), 95437);
});

Deno.test("d7:p1 final", async () => {
  const txt = await Deno.readTextFile("res/input_07.txt");
  assertEquals(
    part1(txt),
    1581595,
  );
});

Deno.test("d7:p2 test", () => {
  assertEquals(part2(input), 24933642);
});

Deno.test("d7:p2 final", async () => {
  const txt = await Deno.readTextFile("res/input_07.txt");
  assertEquals(
    part2(txt),
    1544176,
  );
});
