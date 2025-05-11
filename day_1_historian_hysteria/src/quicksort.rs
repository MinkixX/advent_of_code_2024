use std::fmt::Display;

pub fn sort<T: PartialOrd + Copy + Display>(list: &mut [T]) {
    match partition(list) {
        Some(pivot_index) => {
            let part_list = list.split_at_mut(pivot_index + 1);
            let left_slice_pivot = part_list.0.split_at_mut(pivot_index);

            let left_slice = left_slice_pivot.0;
            let right_slice = part_list.1;

            sort(left_slice);
            sort(right_slice);
        }
        None => {}
    };
}

fn partition<T: PartialOrd + Copy>(list: &mut [T]) -> Option<usize> {
    let length = list.len();

    if length < 2 {
        return None;
    }

    let pivot_index = length - 1;
    let pivot_value = list[pivot_index];
    let mut store_index = 0;

    for item_index in 0..pivot_index {
        // If item is less or equal, move left to left side
        if list[item_index].le(&pivot_value) {
            list.swap(store_index, item_index); // Swap with store_index position
            store_index += 1; // Expand left side
        }
    }

    list.swap(store_index, pivot_index);

    Some(store_index)
}
