use ndarray::{Array1, Array2};

pub fn check_all(trees: &Array2<u32>) -> Array2<bool> {
    let mut visible = ndarray::Array2::from_elem(trees.dim(), false);
    let (nrows, ncols) = trees.dim();
    for i in 0..nrows {
        check_axis(trees, &mut visible, i, true);
    }
    for i in 0..ncols {
        check_axis(trees, &mut visible, i, false);
    }
    visible
}

fn check_axis(trees: &Array2<u32>, visible: &mut Array2<bool>, rownum: usize, col: bool) {
    let (trees_to_check, mut visible_to_check) = if col {
        (trees.column(rownum), visible.column_mut(rownum))
    } else {
        (trees.row(rownum), visible.row_mut(rownum))
    };

    let n_elems = trees_to_check.len();
    let mut maxsize = -1;
    for (i, &elem) in trees_to_check.iter().enumerate() {
        if elem as i32 > maxsize {
            visible_to_check[i] = true;
            if elem == 9 {
                break;
            }
            maxsize = elem as i32;
        }
    }
    maxsize = -1;
    for (i, &elem) in trees_to_check.iter().rev().enumerate() {
        if elem as i32 > maxsize {
            visible_to_check[n_elems - 1 - i] = true;
            if elem == 9 {
                break;
            }
            maxsize = elem as i32;
        }
    }
}

pub fn count_all(trees: &Array2<u32>) -> Array2<u32> {
    let mut scores = ndarray::Array2::from_elem(trees.dim(), 1 as u32);
    let (nrows, ncols) = trees.dim();
    for i in 0..nrows {
        count_axis(trees, &mut scores, i, false);
    }
    for i in 0..ncols {
        count_axis(trees, &mut scores, i, true);
    }
    scores
}

fn count_axis(trees: &Array2<u32>, scores: &mut Array2<u32>, rownum: usize, col: bool) {
    let (trees_to_count, mut tree_scores) = if col {
        (trees.column(rownum), scores.column_mut(rownum))
    } else {
        (trees.row(rownum), scores.row_mut(rownum))
    };
    let mut last_seen: Vec<i32> = vec![-1; 10];
    let nelems = trees_to_count.len();

    for (i, &elem) in trees_to_count.iter().enumerate() {
        let to_check = &last_seen[elem as usize..];
        let (_, &location) = to_check
            .iter()
            .enumerate()
            .max_by_key(|&(_idx, &val)| val)
            .unwrap();
        if location == -1 {
            tree_scores[i] = tree_scores[i] * i as u32;
        } else {
            tree_scores[i] = tree_scores[i] * (i - location as usize) as u32
        }
        last_seen[elem as usize] = i as i32;
    }
    last_seen.fill(-1);
    for (i, &elem) in trees_to_count.iter().rev().enumerate() {
        let to_check = &last_seen[elem as usize..];
        let (_, &location) = to_check
            .iter()
            .rev()
            .enumerate()
            .max_by_key(|&(_idx, &val)| val)
            .unwrap();
        if location == -1 {
            tree_scores[nelems - i - 1] = tree_scores[nelems - i - 1] * i as u32;
        } else {
            tree_scores[nelems - i - 1] =
                tree_scores[nelems - i - 1] * (i - location as usize) as u32
        }
        last_seen[elem as usize] = i as i32;
    }
}
