// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    println!("1 + 2 = {}, even via references", add(&1, &2));
}

fn add(n1: &i32, n2: &i32) -> i32 {
    return (*n1) + (*n2);
}

fn eat(s: String) -> bool {
    return s.starts_with('b') && s.contains('a');
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("plural");
    } else {
        println!("singular");
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        println!("I did modify this string");
        s.push_str("s");
    }
}
