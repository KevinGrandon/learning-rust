// A generic struct
struct Pair<XFOO> {
    first: XFOO,
    second: XFOO,
}

// A generic function
fn swap<XFOO>(pair: Pair<XFOO>) -> Pair<XFOO> {
    let Pair { first: first, second: second } = pair;

    Pair { first: second, second: first }
}

// Reimplementing a 2-element tuple as a tuple struct
struct Tuple2<XFOO, U>(XFOO, U);

fn main() {
    // Explicitly specialize `Pair`
    let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };

    // Implicitly specialize `Pair`
    let pair_of_ints = Pair { first: 1i, second: 2 };

    // Explicitly specialize `Tuple2`
    let _tuple: Tuple2<char, int> = Tuple2('R', 2);

    // Explicitly specialize `swap`
    let _swapped_pair_of_chars = swap::<char>(pair_of_chars);
    println!("{} {}", _swapped_pair_of_chars.first, _swapped_pair_of_chars.second);

    // Implicitly specialize `swap`
    let _swapped_pair_of_ints = swap(pair_of_ints);

    println!("{} {}", _swapped_pair_of_ints.first, _swapped_pair_of_ints.second);
}
