use ansi_term::Colour;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!(
        "{}",
        Colour::Cyan
            .bold()
            .paint("Bem-vido ao jogo de adivinhação!")
    );

    let secret_number: u8 = rand::thread_rng().gen_range(0..100);

    loop {
        print!("Selecione um número: ");

        io::stdout().flush().unwrap();

        let mut number = String::new();

        match io::stdin().read_line(&mut number) {
            Ok(_) => (),
            Err(_) => println!("Não foi possível lé o número!"),
        };

        let number = match number.trim().parse::<u8>() {
            Ok(number) => number,
            Err(number) => {
                println!("{} não é um número!", number);
                continue;
            }
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("{}", Colour::Red.paint("Tente um número maior!")),
            Ordering::Greater => println!("{}", Colour::Red.paint("Tente um número menor!")),
            Ordering::Equal => {
                println!("{}", Colour::Green.paint("Você acertou!"));
                break;
            }
        }
    }
}
