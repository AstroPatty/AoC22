use std::fs;
mod map;
fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let map = map::PathMap::try_from(data).unwrap();
    let paths_p2 = map.shortest_paths(map.end, None, Some(50), Some(1));
    let paths_p1 = map.shortest_paths(map.start, Some(map.end), None, None);

    let lowest_elevations = map
        .elevations
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == 0)
        .map(|(i, _)| i);

    let mut min = usize::MAX;
    for i in lowest_elevations {
        let dist = paths_p2.get(i).unwrap();
        if *dist < min {
            min = *dist;
        }
    }

    println!("{:?}", paths_p1.get(map.end).unwrap());
    println!("{:?}", min);
}
