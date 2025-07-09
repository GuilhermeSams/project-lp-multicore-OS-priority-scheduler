//! Módulo com exemplos pré-configurados para testar o sistema

use crate::sistema::{Sistema, Processo, Recurso, AlgoritmoEscalonamento};
use rand::Rng;
use rand::thread_rng;

pub fn exemplo_round_robin() -> Sistema {
    let mut sistema = Sistema::new(2, 3, AlgoritmoEscalonamento::RoundRobin);

    for i in 1..=4 {
        sistema.adicionar_processo(Processo::new(i, i * 2, 1).necessita_recurso(Recurso::Memoria(1024), 1));
    }

    sistema
}

pub fn exemplo_prioridade() -> Sistema {
    let mut sistema = Sistema::new(2, 3, AlgoritmoEscalonamento::Prioridade);

    let prioridades = [3, 1, 4, 2];
    for (i, &pri) in prioridades.iter().enumerate() {
        sistema.adicionar_processo(Processo::new(i as u32 + 1, 4, pri).necessita_recurso(Recurso::Memoria(1024), 1));
    }

    sistema
}

pub fn exemplo_deadlock() -> Sistema {
    let mut sistema = Sistema::new(2, 3, AlgoritmoEscalonamento::RoundRobin);

    // Configura recursos limitados (substitui os valores padrão)
    sistema.recursos_disponiveis.clear();
    sistema.recursos_disponiveis.insert(Recurso::Impressora, 1);
    sistema.recursos_disponiveis.insert(Recurso::Scanner, 1);
    sistema.recursos_disponiveis.insert(Recurso::Disco, 3);
    sistema.recursos_disponiveis.insert(Recurso::Memoria(1024), 8);

    // Processo 1: precisa de impressora e depois scanner
    sistema.adicionar_processo(
        Processo::new(1, 5, 1)
            .necessita_recurso(Recurso::Impressora, 1)
            .necessita_recurso(Recurso::Scanner, 1)
    );

    // Processo 2: precisa de scanner e depois impressora
    sistema.adicionar_processo(
        Processo::new(2, 5, 1)
            .necessita_recurso(Recurso::Scanner, 1)
            .necessita_recurso(Recurso::Impressora, 1)
    );

    sistema
}

/// Exemplo com múltiplos núcleos e processos
pub fn exemplo_multinucleo() -> Sistema {
    let mut sistema = Sistema::new(4, 2, AlgoritmoEscalonamento::ShortestJobFirst);

    let mut rng = thread_rng();
    for i in 1..=8 {
        let tempo = rng.gen_range(1..10);
        sistema.adicionar_processo(
            Processo::new(i, tempo, 1)
                .necessita_recurso(Recurso::Memoria(1024), 1)
        );
    }

    sistema
}

/// Exemplo complexo com múltiplos tipos de recursos
pub fn exemplo_complexo() -> Sistema {
    let mut sistema = Sistema::new(3, 4, AlgoritmoEscalonamento::RoundRobin);

    // Processos com diferentes necessidades de recursos
    sistema.adicionar_processo(
        Processo::new(1, 6, 2)
            .necessita_recurso(Recurso::Memoria(1024), 2)
            .necessita_recurso(Recurso::Disco, 1)
    );

    sistema.adicionar_processo(
        Processo::new(2, 4, 1)
            .necessita_recurso(Recurso::Memoria(1024), 1)
            .necessita_recurso(Recurso::Impressora, 1)
    );

    sistema.adicionar_processo(
        Processo::new(3, 8, 3)
            .necessita_recurso(Recurso::Memoria(1024), 4)
            .necessita_recurso(Recurso::Scanner, 1)
    );

    sistema.adicionar_processo(
        Processo::new(4, 3, 2)
            .necessita_recurso(Recurso::Memoria(1024), 1)
    );

    sistema
}