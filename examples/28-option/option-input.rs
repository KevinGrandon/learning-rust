use std::io;

fn main() {
    println!("INPUT:");
    let mut reader = io::stdin();

    let input = reader.read_line().ok().expect("Failed to read line");
    let input_num: Option<int> = from_str(input.as_slice().trim());

    match input_num {
        Some(number) => println!("{:d}", number),
        None         => println!("Hey, put in a number.")
    }
}
