fn main() {
    let s1 = "you";
    takes_ownership(s1);
    println!("{s1}");
}

fn takes_ownership(some_string: &str) {
    println!("{some_string}");
}