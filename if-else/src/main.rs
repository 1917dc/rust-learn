fn main() {
    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("number value is {number}");

    if condition {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
}
