use std::collections::HashSet;

// 1. Count the paths
// 2. Count paths with a new rule: one small cave can be visited twice
pub fn run(input: &str) -> (u32, u32) {
    let pt1 = {
        let graph = parse(input);
        let mut pf = Pathfinder::new(&graph);
        pf.count()
    };

    let pt2 = {
        let graph = parse(input);
        let mut pf = Pathfinder::new(&graph);
        pf.count_extra()
    };

    (pt1, pt2)
}

#[derive(Debug)]
struct CaveGraph<'a> {
    nodes: Vec<&'a str>,
    sizes: Vec<CaveSize>,
    edges: Vec<(usize, usize)>,
}

#[derive(Debug, PartialEq)]
enum CaveSize {
    Big,
    Small,
}

impl<'a> CaveGraph<'a> {
    fn new() -> Self {
        CaveGraph {
            nodes: vec![],
            sizes: vec![],
            edges: vec![],
        }
    }

    fn add_node(&mut self, n: &'a str) {
        if !self.nodes.contains(&n) {
            self.nodes.push(n);
            self.sizes.push(match n.chars().all(|c| c.is_uppercase()) {
                true => CaveSize::Big,
                false => CaveSize::Small,
            });
        }
    }

    fn add_edge(&mut self, n1: &'a str, n2: &'a str) {
        if let (Some(p1), Some(p2)) = (
            self.nodes.iter().position(|node| *node == n1),
            self.nodes.iter().position(|node| *node == n2),
        ) {
            if let None = self
                .edges
                .iter()
                .find(|e| (e.0 == p1 && e.1 == p2) || (e.1 == p1 && e.0 == p2))
            {
                self.edges.push((p1, p2));
            }
        }
    }

    fn get_index(&self, n: &str) -> (usize, &CaveSize) {
        let i = self.nodes.iter().position(|node| *node == n).unwrap();
        (i, &self.sizes[i])
    }

    fn get_exits(&self, i: &usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|e| {
                if e.0 == *i || e.1 == *i {
                    Some(if e.0 == *i { e.1 } else { e.0 })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Pathfinder<'a> {
    graph: &'a CaveGraph<'a>,
    start: usize,
    end: usize,
    paths: HashSet<String>,
}

impl<'a> Pathfinder<'a> {
    fn new(graph: &'a CaveGraph) -> Self {
        let (start, _) = graph.get_index("start");
        let (end, _) = graph.get_index("end");
        Pathfinder {
            graph,
            start,
            end,
            paths: HashSet::new(),
        }
    }

    fn count(&mut self) -> u32 {
        self.visit(self.start, &vec![], &String::default(), None);
        self.paths.len() as u32
    }

    fn count_extra(&mut self) -> u32 {
        let start = self.start.clone();
        let end = self.end.clone();
        let extra_visits = self
            .graph
            .sizes
            .iter()
            .enumerate()
            .filter(|&(i, s)| *s == CaveSize::Small && (i != start && i != end));
        for (extra, _) in extra_visits {
            self.visit(self.start, &vec![], &String::default(), Some(extra));
        }
        self.paths.len() as u32
    }

    fn visit(&mut self, index: usize, visited: &Vec<usize>, path: &String, extra: Option<usize>) {
        let p = format!("{}-{}", path, self.graph.nodes[index]);
        if index == self.end {
            self.paths.insert(p);
        } else {
            let mut v = visited.clone();
            if self.graph.sizes[index] == CaveSize::Small {
                v.push(index.clone());
            }
            for n in self
                .graph
                .get_exits(&index)
                .iter()
                .filter(|n| !visit_check(n, &v, extra))
            {
                self.visit(*n, &v, &p, extra);
            }
        }
    }
}

fn visit_check(index: &usize, visits: &Vec<usize>, extra: Option<usize>) -> bool {
    if let Some(x) = extra {
        if *index == x {
            return visits.iter().filter(|v| **v == x).count() >= 2;
        }
    }
    visits.contains(index)
}

fn parse(input: &str) -> CaveGraph {
    input.lines().fold(CaveGraph::new(), |mut cg, line| {
        let nodes = line.split('-').collect::<Vec<_>>();
        cg.add_node(nodes[0]);
        cg.add_node(nodes[1]);
        cg.add_edge(nodes[0], nodes[1]);
        cg
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!((10, 36), run(input));

        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        assert_eq!((226, 3509), run(input));
    }
}
