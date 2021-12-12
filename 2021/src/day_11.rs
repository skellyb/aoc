use nannou::prelude::*;
use std::collections::HashSet;

// 1. Count the flashes
// 2. First sync step
pub fn run(input: &str) -> (u32, u32) {
    let pt1 = {
        let grid_width = input.lines().next().unwrap().len();
        let points: Vec<u8> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let mut grid = OctoGrid::new(points, grid_width);
        for _ in 0..100 {
            grid.tick();
        }
        grid.flash_count
    };

    let pt2 = {
        let grid_width = input.lines().next().unwrap().len();
        let points: Vec<u8> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let mut grid = OctoGrid::new(points, grid_width);
        let mut step = 0;
        loop {
            step += 1;
            if grid.tick() {
                break;
            }
        }
        step
    };

    (pt1, pt2)
}

#[derive(Debug)]
struct OctoGrid {
    points: Vec<u8>,
    neighbors: Vec<[Option<usize>; 8]>,
    flashed: HashSet<usize>,
    flash_count: u32,
}

impl OctoGrid {
    fn new(points: Vec<u8>, width: usize) -> Self {
        let height = points.len() / width;
        let mut neighbors = Vec::new();
        for i in 0..points.len() {
            let x = i % width;
            let y = i / width;
            // Indices of neighboring points,starting with North,
            // continuing around clock-wise
            neighbors.push([
                // N
                if y == 0 { None } else { Some(i - width) },
                // NW
                if y == 0 || x == width - 1 {
                    None
                } else {
                    Some(i - width + 1)
                },
                // W
                if x == width - 1 { None } else { Some(i + 1) },
                // SW
                if x == width - 1 || y >= height - 1 {
                    None
                } else {
                    Some(i + width + 1)
                },
                // S
                if y >= height - 1 {
                    None
                } else {
                    Some(i + width)
                },
                // SE
                if x == 0 || y >= height - 1 {
                    None
                } else {
                    Some(i + width - 1)
                },
                // E
                if x == 0 { None } else { Some(i - 1) },
                // NE
                if x == 0 || y == 0 {
                    None
                } else {
                    Some(i - width - 1)
                },
            ])
        }
        Self {
            points,
            neighbors,
            flashed: HashSet::new(),
            flash_count: 0,
        }
    }

    /// Each tick returns true if the flashes are in sync
    fn tick(&mut self) -> bool {
        self.flashed.clear();
        for i in 0..self.points.len() {
            self.inc(i);
        }
        self.flashed.len() == 100
    }

    fn inc(&mut self, i: usize) {
        if self.points[i] != 0 || !self.flashed.contains(&i) {
            self.points[i] += 1;
        }
        if self.points[i] > 9 {
            self.points[i] = 0;
            self.flashed.insert(i);
            self.flash_count += 1;
            self.neighbors[i].into_iter().for_each(|n| {
                if let Some(neighbor) = n {
                    self.inc(neighbor);
                }
            })
        }
    }
}

// Visualization

pub fn run_app() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    grid: OctoGrid,
    seed: Vec<f32>,
    wait: u32,
}

fn model(_app: &App) -> Model {
    let input = include_str!("../res/day-11.txt");
    let points: Vec<u8> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let grid = OctoGrid::new(points, 10);
    Model {
        grid,
        wait: 0,
        seed: (0..100)
            .into_iter()
            .map(|_| random_f32() * 10.0 - 5.0)
            .collect(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.wait > 100 {
        model.grid.tick();
        model.wait = 0;
    } else {
        model.wait += update.since_last.as_millis() as u32;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(MIDNIGHTBLUE);
    let grid_width = 10;
    for i in 0..100 {
        let (size, alpha) = if model.grid.points[i] == 0 {
            (25.0, 0.95)
        } else if model.grid.points[i] > 7 {
            (
                10.0 + (model.grid.points[i] * 2) as f32,
                map_range(model.grid.points[i], 0, 9, 0.1, 0.9),
            )
        } else {
            (
                5.0 + (model.grid.points[i] * 2) as f32,
                map_range(model.grid.points[i], 0, 9, 0.1, 0.5),
            )
        };
        let x = (i % grid_width) as f32 * 30.0 - 150.0 + (app.time.sin() * model.seed[i]);
        let y = (i / grid_width) as f32 * 30.0 - 150.0 + (app.time.cos() * model.seed[i]);
        draw.ellipse()
            .color(rgba(1.0, 1.0, 1.0, alpha))
            .w_h(size, size)
            .x_y(x, y);
        if model.grid.points[i] == 0 {
            draw.ellipse()
                .color(rgba(1.0, 1.0, 1.0, 0.1))
                .stroke(rgba(1.0, 1.0, 1.0, 0.4))
                .stroke_weight(1.0)
                .w_h(50.0, 50.0)
                .x_y(x, y);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!((1656, 195), run(input));
    }
}
