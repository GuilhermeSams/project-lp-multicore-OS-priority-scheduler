//! Ponto de entrada principal do simulador de escalonamento de processos

mod sistema;
mod examples;
extern crate rand;
use crate::sistema::{Sistema, Processo, Recurso, AlgoritmoEscalonamento};
use crate::examples::*;
use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    println!("=== GERENCIADOR DE TAREFAS - SIMULADOR DE ESCALONAMENTO ===");
    println!("Versão 2.0 - Modo Interativo\n");

    // Menu de configuração
    let config = configurar_sistema();
    
    // Executar o gerenciador de tarefas
    executar_gerenciador_tarefas(config);
}

fn configurar_sistema() -> Sistema {
    println!("=== CONFIGURAÇÃO DO SISTEMA ===");
    
    // Configurar número de núcleos
    let num_nucleos = ler_entrada_usize("Número de núcleos (1-16): ", 1, 16, 4) as u32;
    
    // Configurar algoritmo
    let algoritmo = escolher_algoritmo();
    
    // Configurar quantum (se aplicável)
    let quantum = if matches!(algoritmo, AlgoritmoEscalonamento::RoundRobin) {
        ler_entrada_usize("Quantum (1-10): ", 1, 10, 3) as u32
    } else {
        1
    };
    
    // Configurar taxa de chegada de processos
    let taxa_chegada = ler_entrada_usize("Taxa de chegada de processos (1-100 por segundo): ", 1, 100, 20) as u32;
    
    println!("\nSistema configurado:");
    println!("- Núcleos: {}", num_nucleos);
    println!("- Algoritmo: {}", algoritmo);
    println!("- Quantum: {}", quantum);
    println!("- Taxa de chegada: {} processos/segundo", taxa_chegada);
    
    let mut sistema = Sistema::new(num_nucleos, quantum, algoritmo);
    sistema.taxa_chegada_processos = taxa_chegada;
    
    sistema
}

fn escolher_algoritmo() -> AlgoritmoEscalonamento {
    println!("\nEscolha o algoritmo de escalonamento:");
    println!("1. Round Robin");
    println!("2. Por Prioridade");
    println!("3. Shortest Job First");
    
    loop {
        let escolha = ler_entrada_usize("Opção (1-3): ", 1, 3, 1);
        match escolha {
            1 => return AlgoritmoEscalonamento::RoundRobin,
            2 => return AlgoritmoEscalonamento::Prioridade,
            3 => return AlgoritmoEscalonamento::ShortestJobFirst,
            _ => println!("Opção inválida!"),
        }
    }
}

fn executar_gerenciador_tarefas(mut sistema: Sistema) {
    println!("\n=== GERENCIADOR DE TAREFAS ATIVO ===");
    println!("Comandos disponíveis:");
    println!("- Pressione ENTER para continuar");
    println!("- Digite 'q' para sair");
    println!("- Digite 's' para mostrar estatísticas");
    println!("- Digite 'p' para pausar/continuar");
    println!("- Digite 'a' para adicionar processo manualmente");
    println!("=====================================\n");
    
    let mut pausado = false;
    let mut contador_processos = 0;
    let mut ultima_geracao = std::time::Instant::now();
    
    loop {
        // Verificar entrada do usuário (não-bloqueante)
        if let Some(comando) = verificar_comando() {
            match comando.as_str() {
                "q" => {
                    println!("\nFinalizando gerenciador de tarefas...");
                    break;
                },
                "s" => {
                    sistema.mostrar_estatisticas_detalhadas();
                    continue;
                },
                "p" => {
                    pausado = !pausado;
                    println!("Sistema {}", if pausado { "PAUSADO" } else { "CONTINUANDO" });
                    continue;
                },
                "a" => {
                    adicionar_processo_manual(&mut sistema, &mut contador_processos);
                    continue;
                },
                _ => {}
            }
        }
        
        if pausado {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        
        // Gerar novos processos automaticamente
        let agora = std::time::Instant::now();
        if agora.duration_since(ultima_geracao).as_secs_f64() >= 1.0 / sistema.taxa_chegada_processos as f64 {
            gerar_processo_aleatorio(&mut sistema, &mut contador_processos);
            ultima_geracao = agora;
        }
        
        // Executar um passo do escalonamento
        sistema.escalonar_interativo();
        
        // Mostrar status a cada 10 passos
        if sistema.tempo_global % 10 == 0 {
            mostrar_status_rapido(&sistema, contador_processos);
        }
        
        // Pequena pausa para visualização
        thread::sleep(Duration::from_millis(50));
    }
    
    // Estatísticas finais
    println!("\n=== ESTATÍSTICAS FINAIS ===");
    sistema.mostrar_estatisticas_detalhadas();
}

fn verificar_comando() -> Option<String> {
    let mut buffer = [0u8; 1];
    if io::stdin().read_exact(&mut buffer).is_ok() {
        match buffer[0] as char {
            'q' | 'Q' => Some("q".to_string()),
            's' | 'S' => Some("s".to_string()),
            'p' | 'P' => Some("p".to_string()),
            'a' | 'A' => Some("a".to_string()),
            '\n' => Some("".to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn gerar_processo_aleatorio(sistema: &mut Sistema, contador: &mut u32) {
    let mut rng = rand::thread_rng();
    *contador += 1;
    
    let tempo_total = rng.gen_range(1..20);
    let prioridade = rng.gen_range(1..10);
    
    let mut processo = Processo::new(*contador, tempo_total, prioridade);
    
    // Adicionar recursos aleatórios
    let recursos = [
        (Recurso::Memoria(1024), rng.gen_range(1..4)),
        (Recurso::Disco, rng.gen_range(0..2)),
        (Recurso::Impressora, rng.gen_range(0..1)),
        (Recurso::Scanner, rng.gen_range(0..1)),
    ];
    
    for (recurso, quantidade) in recursos {
        if quantidade > 0 {
            processo = processo.necessita_recurso(recurso, quantidade);
        }
    }
    
    sistema.adicionar_processo(processo);
}

fn adicionar_processo_manual(sistema: &mut Sistema, contador: &mut u32) {
    println!("\n=== ADICIONAR PROCESSO MANUAL ===");
    
    *contador += 1;
    let tempo = ler_entrada_usize("Tempo de execução (1-30): ", 1, 30, 5) as u32;
    let prioridade = ler_entrada_usize("Prioridade (1-10): ", 1, 10, 5) as i32;
    
    let mut processo = Processo::new(*contador, tempo, prioridade);
    
    // Adicionar recursos
    let memoria = ler_entrada_usize("Memória necessária (0-4): ", 0, 4, 1) as u32;
    if memoria > 0 {
        processo = processo.necessita_recurso(Recurso::Memoria(1024), memoria);
    }
    
    sistema.adicionar_processo(processo);
    println!("Processo {} adicionado com sucesso!", *contador);
}

fn mostrar_status_rapido(sistema: &Sistema, total_processos: u32) {
    let processos_prontos = sistema.processos.len();
    let processos_bloqueados = sistema.processos_bloqueados.len();
    let processos_executando = sistema.nucleos.iter()
        .filter(|n| n.processo_atual.is_some()).count();
    
    print!("\r[T={:4}] Prontos: {:3} | Executando: {:3} | Bloqueados: {:3} | Total: {:4}",
           sistema.tempo_global, processos_prontos, processos_executando, processos_bloqueados, total_processos);
    io::stdout().flush().unwrap();
}

fn ler_entrada_usize(prompt: &str, min: usize, max: usize, padrao: usize) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
        
        let input = input.trim();
        if input.is_empty() {
            return padrao;
        }
        
        if let Ok(valor) = input.parse::<usize>() {
            if valor >= min && valor <= max {
                return valor;
            }
        }
        
        println!("Por favor, digite um número entre {} e {}", min, max);
    }
}