fn main() {
    println!("Quero o Teste Um.");

    teste_um();
    add_numbers(20,30);

    let numero: i32 ={
        let x: i32 = 3;
        x + 1
    };
    println!("{}",numero);
    add_numbers2(12,10);
}

fn add_numbers2(x:i32 , y:i32) -> i32{
   let result = x + y;
   if result > 10 {
    return result -10
   };
   result
}



fn teste_um(){

    println!("1,2,3 chamando teste um");

}

fn add_numbers(x: i32 , y: i32){

    println!("A soma é : {}", x + y)
}