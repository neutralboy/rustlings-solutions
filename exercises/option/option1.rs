// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.is_some());
}

fn main() {
    print_number(Option::Some(13));
    print_number(Option::Some(99));

    let mut numbers: [u16; 5] = [0, 1, 2, 3, 4];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = number_to_add;
    }
}
