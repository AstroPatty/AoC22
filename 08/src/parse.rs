use ndarray;

pub fn parse_lines(data: String) -> ndarray::Array2<u32> {
    let lines = data.lines();
    let mut rows = Vec::new();

    for line in lines {
        let arr: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(arr);
    }
    let nrows = rows.len();
    let ncols = rows[0].len();
    let flat: Vec<u32> = rows.into_iter().flatten().collect();
    let array = ndarray::Array2::from_shape_vec((nrows, ncols), flat).unwrap();
    array
}
