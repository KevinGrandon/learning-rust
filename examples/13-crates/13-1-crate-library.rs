// 13-1-crate-library.rs
pub fn public_function() {
    println!("called 13-1-crate-library's `public_function()`");
}

fn private_function() {
    println!("called 13-1-crate-library's `private_function()`");
}

pub fn indirect_access() {
    print!("called 13-1-crate-library's `indirect_access()`, that\n> ");

    private_function();
}
