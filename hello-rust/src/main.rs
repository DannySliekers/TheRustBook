fn main() {
    println!("Hello, world! Number is {}", add_one_to_test());
    println!("Wtf is this man wtf wtf wtf");
}

fn test() -> i32 {
    let number = 5i32;
    return number;
}

fn add_one_to_test() -> i32 {
    return test() + 1;
}