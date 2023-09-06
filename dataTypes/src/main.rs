fn main() {
    // i para inieros contanto negativos ,pósitivos e zero 
    // u unsigned apenas positivos inteiros
    
    let x:u32 = 2;
    let y:i32 = -2;

    let float_numeros = 10.92;
    let logico_boleano: bool = true;
    let caracteres : char = ':';

    // tuples : tipos diferentes em uma única estrutura.

    // Declarando uma tuple com dois elementos
    let tupla: (i32, char) = (42, 'a');

    // Acessando os elementos da tuple
    let primeiro_elemento = tupla.0;
    let segundo_elemento = tupla.1;

    println!("Primeiro elemento: {}", primeiro_elemento);
    println!("Segundo elemento: {}", segundo_elemento);

    // Arrays 
    let mut array: [i32; 5] = [0, 0, 0, 0, 0]; // Inicializa um array de i32 com 5 elementos, todos definidos como 0.

    println!("{}", array[4]); 

}
