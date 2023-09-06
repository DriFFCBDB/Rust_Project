use std::io;
fn main() {
// integer
   let x: i8 = 12;
   let y: i8 = 10;
 
//floating
   let e: f32 = 12.0;// também é possível fazer assim let x = 12i8
   let f: f32 = 10.7;

   let z = x + y;
   let u = e + f;

   println!("{}",z);
   println!("{}",u);
   
    //Conversão entre tipos iguais de usigned
    let pequeno: u32 = 42;
    let grande: u64 = pequeno as u64;

    println!("Pequeno: {}", pequeno);
    println!("Grande: {}", grande);

    //Conversão entre tipos iguais de integer
        let pequeno: u32 = 42;
        let grande: u64 = pequeno as u64;

        println!("Pequeno: {}", pequeno);
        println!("Grande: {}", grande);
        
        //Conversão de String para Inteiro
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("ler linha");
        
        let int_input: i64 = input.trim().parse().unwrap();
        
        println!("{}", int_input + 2);

}
