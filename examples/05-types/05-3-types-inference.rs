fn main() {
    // Using local inference, the compiler knows that `elem` has type u8
    let elem = 5u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`)

    // Insert `elem` in the vector
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    let character = elem as char;
    // vec.push(character);
    // ^ Breaks shit.

    let asInteger = character as u8 + 1;
    vec.push(asInteger);

    println!("{}", vec);
}
