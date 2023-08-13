pub fn print_odd() {
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut _arr_idx = 0; // _ is a throwaway variable

    // loop {
    //     if arr_idx >= array.len() {
    //         break;
    //     }

    //     if array[arr_idx] % 2 == 0 {
    //         arr_idx += 1;
    //         continue;
    //     }

    //     println!("The value of the array at index {} is {}", arr_idx, array[arr_idx]);
    //     arr_idx += 1;
    // }

    // while arr_idx < array.len() {
    //     if array[arr_idx] % 2 == 0 {
    //         arr_idx += 1;
    //         continue;
    //     }

    //     println!("The value of the array at index {} is {}", arr_idx, array[arr_idx]);
    //     arr_idx += 1;
    // }

    // for val in array.iter() {
    //     if val % 2 == 0 {
    //         continue;
    //     }

    //     println!("The value of the array at index {} is {}", arr_idx, val);
    // }
    for (idx, val) in array.iter().enumerate() {
        if val % 2 == 0 {
            continue;
        }

        println!("The value of the array at index {} is {}", idx, val);
    }
}
