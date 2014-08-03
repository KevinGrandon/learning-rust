fn main() {
    let mut count = 0u;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        // if count == 3 {
        if count % 3 == 0 {
            println!("{} fizz", count);
        }

        // if count == 5 {
        if count % 5 == 0 {
            println!("{} buzz", count);
        }

        if count == 100 {
            break;
        }
    }
}
