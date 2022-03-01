fn main() {
    // podemos usar nossa função aqui
    let x = plus_one(5);

    println!("The value of x is: {}", x)
}

// podemos declarar a função acima ou abaixo do main
// nos argumentos, devemos declarar o tipo
// também declaramos o tipo da saída
// para a função retornar o valor, não devemos usar o ; no termo de retorno
// podemos usar a palavra reservada return também
fn plus_one(x: i32) -> i32 {
    x + 1
}