fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut output = Vec::new();
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            output.push(left[left_index]);
            left_index += 1;
        } else {
            output.push(right[right_index]);
            right_index += 1;
        }
    }
    while left_index < left.len() {
        output.push(left[left_index]);
        left_index += 1;
    }
    while right_index < right.len() {
        output.push(right[right_index]);
        right_index += 1;
    }
    output
}

pub fn merge_sort(array: Vec<i32>) -> Vec<i32> {
    if array.len() == 0 || array.len() == 1 {
        return array;
    }
    let mid = array.len() / 2;
    let left = array[..mid].to_vec();
    let right = array[mid..].to_vec();
    let left_sorted = merge_sort(left);
    let right_sorted = merge_sort(right);
    return merge(left_sorted, right_sorted);
}
