use std::collections::HashSet;

// 1. find low points and calculate risk
// 2. find basin around low points
pub fn run(input: &str) -> (i32, i32) {
    let pt1: i32 = {
        let grid_width = input.lines().next().unwrap().len();
        let points: Vec<u8> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let map = Map::new(points, grid_width);
        map.low_points()
            .iter()
            .map(|i| map.points[*i] as i32 + 1)
            .sum()
    };

    let pt2: i32 = {
        let grid_width = input.lines().next().unwrap().len();
        let points: Vec<u8> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let map = Map::new(points, grid_width);
        let mut basins = map.basins();
        basins.sort();
        basins.reverse();
        basins.iter().take(3).product()
    };

    (pt1, pt2)
}

#[derive(Debug)]
struct Map {
    // values on each point of the grid
    points: Vec<u8>,
    // Indices for N, S, E, and W neighboring points
    neighbors: Vec<[Option<usize>; 4]>,
}

impl Map {
    fn new(points: Vec<u8>, width: usize) -> Self {
        let height = points.len() / width;
        let mut neighbors = Vec::new();
        for i in 0..points.len() {
            let x = i % width;
            let y = i / width;
            neighbors.push([
                // North
                if y == 0 { None } else { Some(i - width) },
                // South
                if y >= height - 1 {
                    None
                } else {
                    Some(i + width)
                },
                // East
                if x == 0 { None } else { Some(i - 1) },
                // West
                if x == width - 1 { None } else { Some(i + 1) },
            ])
        }
        Map { points, neighbors }
    }

    fn low_points(&self) -> Vec<usize> {
        self.points
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                let all_greater = self.neighbors[i]
                    .iter()
                    .all(|dir| dir.is_none() || self.points[dir.unwrap()] > *p);
                if all_greater {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn basins(&self) -> Vec<i32> {
        self.low_points()
            .iter()
            .map(|id| {
                let mut hit = HashSet::new();
                let mut miss = HashSet::new();
                self.explore(id, &mut hit, &mut miss);
                hit.len() as i32
            })
            .collect()
    }

    fn explore(&self, id: &usize, hit: &mut HashSet<usize>, miss: &mut HashSet<usize>) {
        hit.insert(*id);
        // Explore N, S, E, W if they exist, are new and less than 9
        for dir in self.neighbors[*id].iter() {
            if let Some(i) = dir {
                if self.points[*i] < 9 && !miss.contains(i) && !hit.contains(i) {
                    self.explore(i, hit, miss);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!((15, 1134), run(input));
    }
}
