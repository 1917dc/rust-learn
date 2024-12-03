use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // rust usa snake case como padrão
    println!("Secret number game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("The secret number is: {secret_number}");
        println!("Type a number:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        // match é basicamente um switch case
        // nesse caso estou usando ele para tratar exceções
        // assim como na linha 32.
        //
        // o método parse retorna um **enum** 
        // que pode ser "Ok" ou "Err", usando o método
        // match consigo tratar a exceção caso um dos dois seja retornado
        // "Err(_)" é basicamente um catch all

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
