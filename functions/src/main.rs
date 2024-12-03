fn main() {

//    let y = {
//        let x = 3;
//        x * 2
//    };
//
//    println!("y is {y}");

    let x = plus_one(5);
    println!("x is {x}");

}

fn five() -> i32 {
    return 5;
}

fn plus_one(num: i32) -> i32 {
    return num + 1;
}
