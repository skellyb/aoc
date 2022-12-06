export function part1(input: string) {
  return find_packet_end(input, 4);
}

export function part2(input: string) {
  return find_packet_end(input, 14);
}

function find_packet_end(input: string, size: number) {
  let length = 0;
  sliding_window(input, size, (chars, end) => {
    const set = new Set(chars);
    if (set.size === size) {
      length = end;
      return true;
    }
    return false;
  });
  return length;
}

function sliding_window(
  input: string,
  size: number,
  callback: (chars: string[], end: number) => boolean,
) {
  let i = 0;
  const view = [];
  for (const c of input) {
    i++;
    view.push(c);
    if (view.length > size) {
      view.shift();
    }
    if (view.length === size) {
      if (callback(view, i)) {
        break;
      }
    }
  }
}
