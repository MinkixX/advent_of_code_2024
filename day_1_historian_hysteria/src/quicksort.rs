use std::fmt::Display;

pub fn sort<T: PartialOrd + Copy + Display>(list: &mut [T]) {
    match partition(list) {
        Some(pivot_index) => {
            println!("\nList 1: ");

            let part_list = list.split_at_mut(pivot_index + 1);
            let left_slice_pivot = part_list.0.split_at_mut(pivot_index);

            let left_slice = left_slice_pivot.0;
            let pivot = left_slice_pivot.1;
            let right_slice = part_list.1;

            for i in 0..left_slice.len() {
                print!("{} ", left_slice[i]);
            }

            println!("\nPivot: {}\nList 2: ", pivot[0]);

            for i in 0..right_slice.len() {
                print!("{} ", right_slice[i]);
            }

            println!("\n");

            sort(left_slice);
            sort(right_slice);
        }
        None => {}
    };
}

fn partition<T: PartialOrd + Copy>(list: &mut [T]) -> Option<usize> {
    let length = list.len();

    if length > 1 {
        let pivot_index = length - 1;
        let pivot = list[pivot_index];
        let mut low_index = 0;
        let mut item_index = 1;

        while item_index < length {
            if list[item_index].le(&pivot) {
                list.swap(low_index, item_index);
                low_index += 1;
            }

            item_index += 1;
        }

        return Some(low_index - 1);
    }

    None
}
