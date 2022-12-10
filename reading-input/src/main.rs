use std::io;

fn main() {
    let mut name = String::new();

    println!("Please informe your name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Erro encontrado ao pegar o input");

    println!("Ola {name}")
}

