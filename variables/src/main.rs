fn main() {

    // forma errada de re-declarar variáveis:
    // x = x (só funciona para variáveis mutáveis)
    let x = 6;
    println!("x is {x}");

    // shadowing variables
    // também posso fazer let x = x * 5!!
    let x = 5;
    println!("x is {x}");

    // constantes são **sempre** imutáveis diferentemente de variáveis.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // inner scope e shadowing variables:
    let x = x + 1;

    {
        // isso é um inner scope
        let x = x * 2;
        println!("x is {x}");
    }

    // a principal diferença de usar shadowing e mut
    // é que variáveis mutáveis podem ser mudadas a qualquer momento
    // enquanto variáveis normais voltam a ser imutáveis após as alterações
    // basicamente, estamos criando uma nova variável, porém com o mesmo nome.
    println!("x is {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces len is {spaces}");

    // forma errada de redeclarar variáveis:
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len();
    //
    // **erro de compilação, pois não é possível mutar o tipo da variável.**
    // apenas é possível fazer isso com as variáveis normais pois elas estão sendo
    // recriadas em toda re-declaração
    
    // preciso definir o tipo com antecedência
    // let guess = "42".parse().expect("Failed to parse!");

    let guess: u32 = "42".parse().expect("Failed to parse!");
}
