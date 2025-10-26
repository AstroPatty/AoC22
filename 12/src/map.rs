#[derive(Debug)]
pub struct PathMap {
    pub start: usize,
    pub end: usize,
    width: usize,
    pub elevations: Vec<usize>,
}

impl PathMap {
    fn neighbors(&self, idx: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        if idx % self.width != 0 {
            // not on left edge
            neighbors.push(idx - 1)
        }
        if (idx + 1) % self.width != 0 {
            neighbors.push(idx + 1)
        }
        if idx >= self.width {
            neighbors.push(idx - self.width);
        }
        if (self.elevations.len() - idx) > self.width {
            neighbors.push(idx + self.width);
        }
        neighbors
    }
    pub fn accessible_neighbors(&self, idx: usize, up_max: usize, down_max: usize) -> Vec<usize> {
        self.neighbors(idx)
            .into_iter()
            .filter(|&val| {
                let ds = *self.elevations.get(val).unwrap() as isize
                    - *self.elevations.get(idx).unwrap() as isize;
                ds <= up_max as isize && ds >= -(down_max as isize)
            })
            .collect()
    }
    pub fn shortest_paths(
        &self,
        start: usize,
        end: Option<usize>,
        upper_max: Option<usize>,
        lower_max: Option<usize>,
    ) -> Vec<usize> {
        let mut shortest_path: Vec<usize> = vec![usize::MAX; self.elevations.len()];
        let mut visited: Vec<bool> = vec![false; self.elevations.len()];
        let mut current_node = start;
        let end_ = end.unwrap_or(usize::MAX);
        let up_max = upper_max.unwrap_or(1);
        let down_max = lower_max.unwrap_or(50);
        shortest_path[current_node] = 0;
        loop {
            let node_distance = shortest_path[current_node];
            if current_node == end_ || node_distance == usize::MAX {
                // got to the end, or we're checking unreachable nodes
                return shortest_path;
            }

            let neighbors = self.accessible_neighbors(current_node, up_max, down_max);
            for neighbor in neighbors {
                if !visited[neighbor] && shortest_path[neighbor] > node_distance + 1 {
                    shortest_path[neighbor] = node_distance + 1;
                }
            }
            visited[current_node] = true;
            let mut indices: Vec<usize> = (0..shortest_path.len()).collect();
            indices.sort_by_key(|&i| shortest_path[i]);
            for i in indices {
                if !visited[i] {
                    current_node = i;
                    break;
                }
            }
        }
    }
}

impl TryFrom<String> for PathMap {
    type Error = ();
    fn try_from(data: String) -> Result<Self, Self::Error> {
        let mut width = 0;
        let mut start = 0;
        let mut end = 0;
        let mut elevations: Vec<usize> = Vec::new();
        for (li, line) in data.lines().enumerate() {
            if width != 0 && line.len() != width {
                return Err(());
            }
            width = line.len();
            for (ci, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = li * width + ci;
                        elevations.push(0);
                    }
                    'E' => {
                        end = li * width + ci;
                        elevations.push(25);
                    }
                    c => {
                        let val = (c as usize) - ('a' as usize);
                        elevations.push(val);
                    }
                }
            }
        }
        Ok(PathMap {
            start,
            end,
            width,
            elevations,
        })
    }
}
