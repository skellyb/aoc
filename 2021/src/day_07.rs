pub fn run(input: &str) -> (i32, i32) {
    let pt1 = {
        let positions: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();
        let (min, max) = min_max(&positions);
        most_efficient(&positions, min, max, |p1, p2| i32::abs(p1 - p2))
    };

    let pt2 = {
        let positions: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();
        let (min, max) = min_max(&positions);
        most_efficient(&positions, min, max, |p1, p2| {
            let diff = i32::abs(p1 - p2);
            diff * (diff + 1) / 2
        })
    };

    (pt1, pt2)
}

fn min_max(positions: &Vec<i32>) -> (i32, i32) {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    (*min, *max)
}

fn most_efficient<F>(positions: &Vec<i32>, min: i32, max: i32, burn: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let mut lowest = None;
    for pos in min..=max {
        let fuel = positions.iter().fold(0, |acc, p| acc + burn(*p, pos));
        if lowest.is_none() || lowest.unwrap() > fuel {
            lowest = Some(fuel);
        }
    }
    lowest.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!((37, 168), run(input));
    }
}
