/// 1. Simulate fish for 80 days
/// 2. Simulate fish for 256 days
pub fn run(input: &str) -> (u64, u64) {
    let pt1 = {
        let fish: Vec<u8> = input.split(',').map(|n| n.parse::<u8>().unwrap()).collect();
        simulate(fish, 80)
    };

    let pt2 = {
        let fish: Vec<u8> = input.split(',').map(|n| n.parse::<u8>().unwrap()).collect();
        simulate(fish, 256)
    };

    (pt1, pt2)
}

fn simulate(fish: Vec<u8>, days: u64) -> u64 {
    let mut pool: u64;
    let mut state = fish.into_iter().fold([0_u64; 9], |mut acc, val| {
        acc[val as usize] += 1;
        acc
    });
    for _ in 0..days {
        pool = state[0];
        for i in 0..=8 {
            if i == 8 {
                state[i] = pool
            } else {
                state[i] = state[i + 1];
            }
            if i == 6 {
                state[i] += pool;
            }
        }
    }
    state.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "3,4,3,1,2";
        assert_eq!((5934, 26984457539), run(input));
    }
}
