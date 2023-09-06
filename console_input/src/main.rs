use std::io;

fn main() {
 //prelude
    println!("Input saindo");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("falha ao ler a linha");

    println!("{}",input);

}
