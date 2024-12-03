fn main() {
    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        } 
    };

    println!("result is {result}");

    let mut number: u32 = 3;
    println!("number is {number}");

    while number != 0 {
        number -= 1
    }
    println!("number has reached {number}");

    while number < 10 {
        number += 1;
    }
    println!("number has reached {number}");

    let mut index: usize = 0;
    let arr:[u32; 5] = [0,1,2,3,4];

    while index < 5 {
        println!("element is {}", arr[index]);
        index += 1;
    }

    // ao invés de fazermos um loop com um tamanho 5, que pode causar erros
    // posso fazer o loop dentro da propria coleção de arr, da seguinte forma:

    println!("\n");
    for element in arr {
        println!("element is {element}");
    }


    println!("\n");

    for element in (1..4).rev() {
        println!("element is {element}");
    }

}
