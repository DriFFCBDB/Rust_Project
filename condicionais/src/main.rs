//Condicionais e operadores lógicos
//Sempre usar o mesmo tipo de dado ao comparar um lado com o outro
// <,> , <=,>=,!=,==
fn main() {

    let cond = (2 as f32) <= 2.2;

    let cond2 = false && cond;

    //operador negation !(true || cond),esse operador não é o caso de que pelo menos um deles seja verdadeiro.

    println!("{}",cond);

    let doce = "doce-de-leite";

     if  doce == "doce-de-leite"{

        println!("Eu amo doce.")

     }else if doce == "bombom"{

        println!("Eu não gosto desse doce.")

     }else{

        println!("Eu não gosto de doce-de-leite.")
     
     }
}



