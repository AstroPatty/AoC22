use std::collections::HashSet;
use std::fs;

fn is_different(data: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for c in data.chars() {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    return true;
}

fn scan<F>(data: &str, scan_size: usize, test_fn: F) -> Option<usize>
where
    F: Fn(&str) -> bool,
{
    let l = data.len();
    if scan_size > l {
        return None;
    }
    for i in scan_size..l + 1 {
        let slice = &data[i - scan_size..i];
        if test_fn(slice) {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let result_1 = scan(&data, 4, is_different);
    let result_2 = scan(&data, 14, is_different);
    println!("{:?}", result_1);
    println!("{:?}", result_2);
}
