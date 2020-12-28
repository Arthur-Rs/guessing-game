use ansi_term::Colour;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

pub struct Palpite {
    valor: u8
}

impl Palpite {
    pub fn new(valor: u8) -> Palpite {
        if valor < 1 || valor > 100 {
            panic!("O número precisa estar entre 1 e 100");
        }

        Palpite { valor }
    }

    pub fn valor(&self) -> u8 {
        self.valor
    }
}

fn main() {
    println!(
        "{}",
        Colour::Cyan
            .bold()
            .paint("Bem-vido ao jogo de adivinhação!")
    );

    let numero_secreto: u8 = rand::thread_rng().gen_range(0..100);

    loop {
        print!("Selecione um número: ");

        io::stdout().flush().unwrap();

        let mut palpite = String::new();

        match io::stdin().read_line(&mut palpite) {
            Ok(_) => (),
            Err(_) => println!("Não foi possível lé o número!"),
        };

        let palpite = match palpite.trim().parse::<u8>() {
            Ok(palpite) => palpite,
            Err(palpite) => {
                println!("{} não é um número!", palpite);
                continue;
            }
        };

        let palpite = Palpite::new(palpite);

        match palpite.valor().cmp(&numero_secreto) {
            Ordering::Less => println!("{}", Colour::Red.paint("Tente um número maior!")),
            Ordering::Greater => println!("{}", Colour::Red.paint("Tente um número menor!")),
            Ordering::Equal => {
                println!("{}", Colour::Green.paint("Você acertou!"));
                break;
            }
        }
    }
}
