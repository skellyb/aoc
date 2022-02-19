pub fn run(input: &str) -> (i64, i64) {
    let nums = parse(input);
    let mut sum = nums[0].add(&nums[1]);
    sum.reduce();
    println!("{:?}", sum);
    (0, 0)
}

#[derive(Debug)]
struct SFNumber {
    pairs: Vec<(Handle, Handle)>,
    depths: Vec<u8>,
    values: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Handle {
    Pair(usize),
    Value(usize),
}

impl SFNumber {
    fn add(&self, right: &Self) -> Self {
        let mut pairs: Vec<(Handle, Handle)> = self
            .pairs
            .iter()
            .chain(right.pairs.iter())
            .enumerate()
            .map(|(i, (l, r))| {
                if i >= self.pairs.len() {
                    match (l, r) {
                        (Handle::Pair(i1), Handle::Pair(i2)) => (
                            Handle::Pair(i1 + self.pairs.len()),
                            Handle::Pair(i2 + self.pairs.len()),
                        ),
                        (Handle::Value(i1), Handle::Value(i2)) => (
                            Handle::Value(i1 + self.pairs.len()),
                            Handle::Value(i2 + self.pairs.len()),
                        ),
                        (Handle::Pair(i1), Handle::Value(i2)) => (
                            Handle::Pair(i1 + self.pairs.len()),
                            Handle::Value(i2 + self.pairs.len()),
                        ),
                        (Handle::Value(i1), Handle::Pair(i2)) => (
                            Handle::Value(i1 + self.pairs.len()),
                            Handle::Pair(i2 + self.pairs.len()),
                        ),
                    }
                } else {
                    (*l, *r)
                }
            })
            .collect();
        pairs.push((Handle::Pair(0), Handle::Pair(self.pairs.len())));
        let mut depths: Vec<u8> = self
            .depths
            .iter()
            .chain(right.depths.iter())
            .map(|d| d + 1)
            .collect();
        depths.push(1);
        let mut values = self.values.clone();
        values.extend(right.values.iter());
        SFNumber {
            pairs,
            depths,
            values,
        }
    }

    fn reduce(&mut self) {
        if let Some(i) = self.depths.iter().position(|d| *d > 4) {
            // explode
            match self.pairs[i] {
                (Handle::Value(l), Handle::Value(r)) => {
                    // add values to adjacent
                    if l > 0 {
                        self.values[l - 1] += self.values[l];
                    }
                    if r < self.pairs.len() - 1 {
                        self.values[r + 1] += self.values[r];
                    }
                    // replace with zero
                    self.values.remove(l);
                    self.values.remove(r);
                    self.values.insert(l, 0);
                    // update parent pair with handle to zero
                    if let Some(parent) = self
                        .pairs
                        .iter()
                        .position(|(lh, rh)| *lh == Handle::Pair(i) || *rh == Handle::Pair(i))
                    {
                        if self.pairs[parent].0 == Handle::Pair(i) {
                            self.pairs[parent].0 = Handle::Value(l)
                        } else {
                            self.pairs[parent].1 = Handle::Value(l)
                        }
                    }
                    // remove exploded pair and depth
                    self.pairs.remove(i);
                    self.depths.remove(i);
                    // shift all indexes after removal
                    self.shift_pairs(i, -1);
                    self.shift_values(r, -1);

                    for update in 0..self.pairs.len() {
                        match self.pairs[update].0 {
                            Handle::Pair(idx) if idx > r => {
                                self.pairs[update].0 = Handle::Pair(idx - 1)
                            }
                            Handle::Value(idx) if idx > r => {
                                self.pairs[update].0 = Handle::Value(idx - 1)
                            }
                            _ => {}
                        }
                        match self.pairs[update].1 {
                            Handle::Pair(idx) if idx > r => {
                                self.pairs[update].1 = Handle::Pair(idx - 1)
                            }
                            Handle::Value(idx) if idx > r => {
                                self.pairs[update].1 = Handle::Value(idx - 1)
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            self.reduce();
        } else if let Some(i) = self.values.iter().position(|v| *v > 9) {
            // split
            if let Some(p) = self
                .pairs
                .iter()
                .position(|(l, r)| *l == Handle::Value(i) || *r == Handle::Value(i))
            {
                if self.pairs[p].0 == Handle::Value(i) {
                    self.pairs[p].0 = Handle::Pair(self.pairs.len());
                } else {
                    self.pairs[p].1 = Handle::Pair(self.pairs.len());
                }
                self.pairs.push((Handle::Value(i), Handle::Value(i + 1)));
                self.depths.push(self.depths[p] + 1);
            }
            let (down, up) = {
                let rem = self.values[i] % 2;
                let div = self.values[i] / 2;
                if rem != 0 {
                    (div, div + 1)
                } else {
                    (div, div)
                }
            };
            self.values.remove(i);
            self.values.insert(i, up); // right
            self.values.insert(i, down); // left
            self.shift_values(i + 1, 1);
        }
    }

    fn shift_pairs(&mut self, after: usize, amount: i32) {
        for update in 0..self.pairs.len() {
            match self.pairs[update].0 {
                Handle::Pair(idx) if idx > after => {
                    self.pairs[update].0 = Handle::Pair((idx as i32 + amount) as usize)
                }
                _ => {}
            }
            match self.pairs[update].1 {
                Handle::Pair(idx) if idx > after => {
                    self.pairs[update].1 = Handle::Pair((idx as i32 + amount) as usize)
                }
                _ => {}
            }
        }
    }

    fn shift_values(&mut self, after: usize, amount: i32) {
        for update in 0..self.pairs.len() {
            match self.pairs[update].0 {
                Handle::Value(idx) if idx > after => {
                    self.pairs[update].0 = Handle::Value((idx as i32 + amount) as usize)
                }
                _ => {}
            }
            match self.pairs[update].1 {
                Handle::Value(idx) if idx > after => {
                    self.pairs[update].1 = Handle::Value((idx as i32 + amount) as usize)
                }
                _ => {}
            }
        }
    }
}

fn parse(input: &str) -> Vec<SFNumber> {
    input
        .lines()
        .map(|ln| {
            let mut depth = 0;
            let mut depths: Vec<u8> = vec![];
            let mut pairs: Vec<(Option<Handle>, Option<Handle>)> = vec![];
            let mut values: Vec<u8> = vec![];
            let mut queue: Vec<usize> = vec![];
            for c in ln.chars() {
                match c {
                    '[' => {
                        depth += 1;
                        if queue.len() > 0 {
                            let i = *queue.last().unwrap();
                            if pairs[i].0.is_none() {
                                pairs[i].0 = Some(Handle::Pair(pairs.len()));
                            } else if pairs[i].1.is_none() {
                                pairs[i].1 = Some(Handle::Pair(pairs.len()));
                            }
                        }
                        queue.push(pairs.len());
                        pairs.push((None, None));
                        depths.push(depth);
                    }
                    ']' => {
                        depth -= 1;
                        queue.pop();
                    }
                    _ => {
                        if c.is_digit(10) {
                            if let Some(&i) = queue.last() {
                                if pairs[i].0.is_none() {
                                    pairs[i].0 = Some(Handle::Value(values.len()));
                                } else if pairs[i].1.is_none() {
                                    pairs[i].1 = Some(Handle::Value(values.len()));
                                }
                            }
                            let n = c.to_digit(10).unwrap() as u8;
                            values.push(n);
                        }
                    }
                }
            }
            SFNumber {
                pairs: pairs
                    .into_iter()
                    .map(|(l, r)| (l.unwrap(), r.unwrap()))
                    .collect(),
                depths,
                values,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        //         let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        // [[[5,[2,8]],4],[5,[[9,9],0]]]
        // [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        // [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        // [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        // [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        // [[[[5,4],[7,7]],8],[[8,3],8]]
        // [[9,3],[[9,9],[6,[4,9]]]]
        // [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        // [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        // = [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        assert_eq!((4140, 0), run(input));
    }
}
