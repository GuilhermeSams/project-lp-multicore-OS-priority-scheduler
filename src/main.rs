//! Ponto de entrada principal do simulador de escalonamento de processos

mod sistema;
mod examples;
extern crate rand;
use crate::sistema::Sistema;
use crate::examples::*;

fn main() {
    println!("=== SIMULADOR DE ESCALONADOR DE PROCESSOS ===");

    // Menu de exemplos
    let exemplos = [
        ("Round Robin básico", exemplo_round_robin as fn() -> Sistema),
        ("Escalonamento por prioridade", exemplo_prioridade),
        ("Detecção de deadlock", exemplo_deadlock),
        ("Sistema multi-núcleo", exemplo_multinucleo),
        ("Cenário complexo", exemplo_complexo),
    ];

    for (i, (nome, _)) in exemplos.iter().enumerate() {
        println!("{}. {}", i + 1, nome);
    }

    println!("0. Sair");
    println!("\nSelecione um exemplo para executar:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

    if let Ok(escolha) = input.trim().parse::<usize>() {
        match escolha {
            1..=5 => {
                let exemplo = exemplos[escolha - 1].1;
                let mut sistema = exemplo();
                sistema.executar(30);
            },
            0 => println!("Saindo..."),
            _ => println!("Opção inválida!"),
        }
    } else {
        println!("Entrada inválida!");
    }
}