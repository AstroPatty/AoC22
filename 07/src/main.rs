mod filesystem;
use std::fs;
fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut session = filesystem::Session::new();
    session.run_session(data);
    let sizes = session.fs.get_sizes();
    let p1_size: usize = sizes
        .iter()
        .map(|(dirname, size)| {
            if dirname != "/" && *size <= 100000 {
                *size
            } else {
                0 as usize
            }
        })
        .sum();
    println!("Part 1: {:?}", p1_size);
    let disk_size: usize = 70000000;
    let space_required: usize = 30000000;
    let total_size = sizes.get("/").unwrap();
    //println!("{:?}", sizes);
    let free_space = disk_size - total_size;
    let required_additional_space = space_required - free_space;
    let mut folder_size = disk_size;
    for &size in sizes.values() {
        if size > required_additional_space && size < folder_size {
            folder_size = size
        }
    }

    println!("Part 2: {:?}", folder_size);
}

