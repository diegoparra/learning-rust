use std::io;

fn main() {
    let mut name = String::new();

    println!("Digite seu nome: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Erro ao ler o input digitado");

    println!("Ola {name}");
}
