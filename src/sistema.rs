//! Módulo principal que implementa a lógica de escalonamento de processos

use std::collections::{VecDeque, HashMap};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Recurso {
    Impressora,
    Scanner,
    Disco,
    Memoria(u32), // Quantidade em MB
}

impl fmt::Display for Recurso {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Recurso::Impressora => write!(f, "Impressora"),
            Recurso::Scanner => write!(f, "Scanner"),
            Recurso::Disco => write!(f, "Disco"),
            Recurso::Memoria(mb) => write!(f, "Memória({}MB)", mb),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EstadoProcesso {
    Pronto,
    Executando,
    Bloqueado,
    Concluido,
}

impl fmt::Display for EstadoProcesso {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EstadoProcesso::Pronto => write!(f, "Pronto"),
            EstadoProcesso::Executando => write!(f, "Executando"),
            EstadoProcesso::Bloqueado => write!(f, "Bloqueado"),
            EstadoProcesso::Concluido => write!(f, "Concluído"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Processo {
    pub id: u32,
    pub prioridade: i32,
    pub tempo_total: u32,
    pub tempo_restante: u32,
    pub tempo_chegada: u32,
    pub estado: EstadoProcesso,
    pub recursos_alocados: HashMap<Recurso, u32>,
    pub recursos_necessarios: HashMap<Recurso, u32>,
}

impl Processo {
    pub fn new(id: u32, tempo_total: u32, prioridade: i32) -> Self {
        Processo {
            id,
            prioridade,
            tempo_total,
            tempo_restante: tempo_total,
            tempo_chegada: 0,
            estado: EstadoProcesso::Pronto,
            recursos_alocados: HashMap::new(),
            recursos_necessarios: HashMap::new(),
        }
    }

    pub fn necessita_recurso(mut self, recurso: Recurso, quantidade: u32) -> Self {
        self.recursos_necessarios.insert(recurso, quantidade);
        self
    }
}

#[derive(Debug)]
pub struct Nucleo {
    pub id: u32,
    pub processo_atual: Option<Processo>,
    pub tempo_ocioso: u32,
}

impl Nucleo {
    fn new(id: u32) -> Self {
        Nucleo {
            id,
            processo_atual: None,
            tempo_ocioso: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgoritmoEscalonamento {
    RoundRobin,
    Prioridade,
    ShortestJobFirst,
}

impl fmt::Display for AlgoritmoEscalonamento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlgoritmoEscalonamento::RoundRobin => write!(f, "Round Robin"),
            AlgoritmoEscalonamento::Prioridade => write!(f, "Por Prioridade"),
            AlgoritmoEscalonamento::ShortestJobFirst => write!(f, "Shortest Job First"),
        }
    }
}

/// Sistema principal que gerencia todos os componentes
pub struct Sistema {
    pub nucleos: Vec<Nucleo>,
    pub processos: VecDeque<Processo>,
    pub processos_bloqueados: Vec<Processo>,
    pub recursos_disponiveis: HashMap<Recurso, u32>,
    pub algoritmo: AlgoritmoEscalonamento,
    pub tempo_global: u32,
    pub quantum: u32,
    pub taxa_chegada_processos: u32,
}

impl Sistema {
    pub fn new(num_nucleos: u32, quantum: u32, algoritmo: AlgoritmoEscalonamento) -> Self {
        let mut nucleos = Vec::with_capacity(num_nucleos as usize);
        for i in 0..num_nucleos {
            nucleos.push(Nucleo::new(i));
        }

        let recursos = HashMap::from([
            (Recurso::Impressora, 2),
            (Recurso::Scanner, 1),
            (Recurso::Disco, 3),
            (Recurso::Memoria(1024), 8), // 8GB total
        ]);

        Sistema {
            nucleos,
            processos: VecDeque::new(),
            processos_bloqueados: Vec::new(),
            recursos_disponiveis: recursos,
            algoritmo,
            tempo_global: 0,
            quantum,
            taxa_chegada_processos: 20, // padrão
        }
    }

    pub fn adicionar_processo(&mut self, processo: Processo) {
        let mut processo = processo;
        processo.estado = EstadoProcesso::Pronto;
        self.processos.push_back(processo);
    }

    /// Verifica se existe algum deadlock no sistema usando o algoritmo do banqueiro
    pub fn verificar_deadlock(&self) -> bool {
        let mut trabalho = self.recursos_disponiveis.clone();
        let processos = self.processos.iter().chain(self.processos_bloqueados.iter());

        let mut finish = HashMap::new();
        for p in processos.clone() {
            finish.insert(p.id, false);
        }

        loop {
            let mut encontrou = false;

            for p in processos.clone() {
                if !finish[&p.id] {
                    let mut recursos_suficientes = true;

                    for (recurso, &necessario) in &p.recursos_necessarios {
                        let disponivel = trabalho.get(recurso).unwrap_or(&0);
                        if necessario > *disponivel {
                            recursos_suficientes = false;
                            break;
                        }
                    }

                    if recursos_suficientes {
                        encontrou = true;
                        finish.insert(p.id, true);

                        for (recurso, &alocado) in &p.recursos_alocados {
                            *trabalho.entry(*recurso).or_insert(0) += alocado;
                        }
                    }
                }
            }

            if !encontrou {
                break;
            }
        }

        finish.values().any(|&f| !f)
    }

    fn alocar_recursos(&mut self, processo: &Processo) -> bool {
        for (recurso, &necessario) in &processo.recursos_necessarios {
            let disponivel = self.recursos_disponiveis.get(recurso).unwrap_or(&0);
            if necessario > *disponivel {
                return false;
            }
        }

        for (recurso, &necessario) in &processo.recursos_necessarios {
            *self.recursos_disponiveis.get_mut(recurso).unwrap() -= necessario;
        }

        true
    }

    fn liberar_recursos(&mut self, processo: &Processo) {
        for (recurso, &alocado) in &processo.recursos_alocados {
            *self.recursos_disponiveis.entry(*recurso).or_insert(0) += alocado;
        }
    }

    pub fn escalonar(&mut self) {
        // Verificação de deadlock (sem problemas de borrowing)
        if self.tempo_global % 10 == 0 && self.verificar_deadlock() {
            println!("[!] Deadlock detectado no tempo {}! Tomando ações corretivas...", self.tempo_global);
            if let Some(processo) = self.processos.pop_front() {
                println!("[!] Processo {} terminado para resolver deadlock", processo.id);
                self.liberar_recursos(&processo);
            }
        }

        // Fase 1: Processar núcleos
        let mut processos_concluidos = Vec::new();
        let mut processos_preemptados = Vec::new();

        for nucleo in &mut self.nucleos {
            if let Some(processo) = nucleo.processo_atual.take() {
                if processo.tempo_restante == 0 {
                    processos_concluidos.push(processo);
                } else {
                    let mut processo = processo;
                    processo.tempo_restante -= 1;

                    if self.algoritmo == AlgoritmoEscalonamento::RoundRobin &&
                        (self.tempo_global - processo.tempo_chegada) % self.quantum == 0 {
                        processo.estado = EstadoProcesso::Pronto;
                        processos_preemptados.push(processo);
                    } else {
                        processo.estado = EstadoProcesso::Executando;
                        nucleo.processo_atual = Some(processo);
                    }
                }
            }
        }

        // Liberar recursos fora do loop dos núcleos
        for processo in processos_concluidos {
            println!("[T={}] Processo {} concluído", self.tempo_global, processo.id);
            self.liberar_recursos(&processo);
        }

        // Recolocar processos preemptados
        for processo in processos_preemptados {
            println!("[T={}] Processo {} preemptado", self.tempo_global, processo.id);
            // Liberar recursos do processo preemptado
            self.liberar_recursos(&processo);
            // Criar uma cópia sem recursos alocados para a fila
            let mut processo_sem_recursos = processo;
            processo_sem_recursos.recursos_alocados.clear();
            self.processos.push_back(processo_sem_recursos);
        }

        // Fase 2: Atribuir novos processos
        let mut processos_para_atribuir = Vec::new();
        
        // Primeiro, coletamos os processos que podem ser atribuídos
        for _ in 0..self.nucleos.len() {
            if let Some(processo) = self.escolher_proximo_processo() {
                let pode_alocar = processo.recursos_necessarios.iter()
                    .all(|(r, &q)| self.recursos_disponiveis.get(r).map_or(false, |&d| d >= q));
                
                if pode_alocar {
                    processos_para_atribuir.push(processo.clone());
                    // Remover o processo da fila
                    self.processos.pop_front();
                } else {
                    // Processo não pode ser alocado, vamos para o próximo
                    break;
                }
            } else {
                break;
            }
        }

        // Agora atribuímos os processos aos núcleos
        let mut processo_index = 0;
        for nucleo in &mut self.nucleos {
            if nucleo.processo_atual.is_none() && processo_index < processos_para_atribuir.len() {
                let mut processo = processos_para_atribuir[processo_index].clone();
                
                // Alocar recursos com verificação de segurança
                for (recurso, &quantidade) in &processo.recursos_necessarios {
                    if let Some(disponivel) = self.recursos_disponiveis.get_mut(recurso) {
                        if *disponivel >= quantidade {
                            *disponivel -= quantidade;
                            // Mover recursos para recursos_alocados
                            processo.recursos_alocados.insert(*recurso, quantidade);
                        } else {
                            // Não há recursos suficientes, não alocar
                            continue;
                        }
                    }
                }

                processo.estado = EstadoProcesso::Executando;
                nucleo.processo_atual = Some(processo);
                println!("[T={}] Núcleo {}: Processo {} iniciado",
                         self.tempo_global, nucleo.id, nucleo.processo_atual.as_ref().unwrap().id);
                
                processo_index += 1;
            }
        }

        // Processos que não puderam ser atribuídos voltam para a fila
        for processo in processos_para_atribuir.into_iter().skip(processo_index) {
            let processo_id = processo.id;
            let mut processo = processo;
            processo.estado = EstadoProcesso::Bloqueado;
            // Liberar recursos se houver algum alocado
            if !processo.recursos_alocados.is_empty() {
                self.liberar_recursos(&processo);
                processo.recursos_alocados.clear();
            }
            self.processos_bloqueados.push(processo);
            println!("[T={}] Processo {} bloqueado", self.tempo_global, processo_id);
        }

        // Fase 3: Verificar processos bloqueados
        let mut i = 0;
        while i < self.processos_bloqueados.len() {
            let pode_alocar = self.processos_bloqueados[i].recursos_necessarios.iter()
                .all(|(r, &q)| self.recursos_disponiveis.get(r).map_or(false, |&d| d >= q));

            if pode_alocar {
                let processo = self.processos_bloqueados.remove(i);
                println!("[T={}] Processo {} desbloqueado", self.tempo_global, processo.id);
                self.processos.push_front(processo);
            } else {
                i += 1;
            }
        }

        self.tempo_global += 1;
    }


    fn pode_alocar_recursos(&self, processo: &Processo) -> bool {
        processo.recursos_necessarios.iter()
            .all(|(r, &q)| self.recursos_disponiveis.get(r).map_or(false, |&d| d >= q))
    }

    fn escolher_proximo_processo(&self) -> Option<&Processo> {
        match self.algoritmo {
            AlgoritmoEscalonamento::RoundRobin => self.processos.front(),
            AlgoritmoEscalonamento::Prioridade => {
                self.processos.iter().max_by_key(|p| p.prioridade)
            },
            AlgoritmoEscalonamento::ShortestJobFirst => {
                self.processos.iter().min_by_key(|p| p.tempo_restante)
            },
        }
    }

    pub fn executar(&mut self, passos: u32) {
        println!("Iniciando sistema com {} núcleos, algoritmo {}, quantum {}",
                 self.nucleos.len(), self.algoritmo, self.quantum);

        for _ in 0..passos {
            if self.processos.is_empty() && self.processos_bloqueados.is_empty() &&
                self.nucleos.iter().all(|n| n.processo_atual.is_none()) {
                println!("Todos os processos foram concluídos!");
                break;
            }

            self.escalonar();
        }

        println!("Simulação concluída no tempo {}", self.tempo_global);
        self.mostrar_estatisticas();
    }

    pub fn mostrar_estatisticas(&self) {
        println!("\n=== Estatísticas ===");
        for nucleo in &self.nucleos {
            println!("Núcleo {}: tempo ocioso = {}", nucleo.id, nucleo.tempo_ocioso);
        }

        println!("\nRecursos disponíveis:");
        for (recurso, quantidade) in &self.recursos_disponiveis {
            println!("  {}: {}", recurso, quantidade);
        }
    }

    pub fn mostrar_estatisticas_detalhadas(&self) {
        println!("\n=== ESTATÍSTICAS DETALHADAS ===");
        println!("Tempo global: {}", self.tempo_global);
        println!("Algoritmo: {}", self.algoritmo);
        println!("Quantum: {}", self.quantum);
        println!("Taxa de chegada: {} processos/segundo", self.taxa_chegada_processos);
        
        println!("\n=== NÚCLEOS ===");
        for nucleo in &self.nucleos {
            let status = if nucleo.processo_atual.is_some() {
                format!("Executando P{}", nucleo.processo_atual.as_ref().unwrap().id)
            } else {
                "Ocioso".to_string()
            };
            println!("Núcleo {}: {} (tempo ocioso: {})", nucleo.id, status, nucleo.tempo_ocioso);
        }
        
        println!("\n=== FILAS DE PROCESSOS ===");
        println!("Prontos: {} processos", self.processos.len());
        for (i, processo) in self.processos.iter().take(5).enumerate() {
            println!("  {}. P{} (prioridade: {}, tempo restante: {})", 
                     i+1, processo.id, processo.prioridade, processo.tempo_restante);
        }
        if self.processos.len() > 5 {
            println!("  ... e mais {} processos", self.processos.len() - 5);
        }
        
        println!("Bloqueados: {} processos", self.processos_bloqueados.len());
        for (i, processo) in self.processos_bloqueados.iter().take(3).enumerate() {
            println!("  {}. P{} (prioridade: {}, tempo restante: {})", 
                     i+1, processo.id, processo.prioridade, processo.tempo_restante);
        }
        if self.processos_bloqueados.len() > 3 {
            println!("  ... e mais {} processos", self.processos_bloqueados.len() - 3);
        }
        
        println!("\n=== RECURSOS DISPONÍVEIS ===");
        for (recurso, quantidade) in &self.recursos_disponiveis {
            println!("  {}: {}", recurso, quantidade);
        }
    }

    pub fn escalonar_interativo(&mut self) {
        // Versão simplificada do escalonar para modo interativo
        self.escalonar();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processo_new() {
        let processo = Processo::new(1, 100, 5);
        
        assert_eq!(processo.id, 1);
        assert_eq!(processo.prioridade, 5);
        assert_eq!(processo.tempo_total, 100);
        assert_eq!(processo.tempo_restante, 100);
        assert_eq!(processo.tempo_chegada, 0);
        assert_eq!(processo.estado, EstadoProcesso::Pronto);
        assert!(processo.recursos_alocados.is_empty());
        assert!(processo.recursos_necessarios.is_empty());
    }

    #[test]
    fn test_processo_necessita_recurso() {
        let processo = Processo::new(1, 100, 5)
            .necessita_recurso(Recurso::Impressora, 1)
            .necessita_recurso(Recurso::Memoria(256), 2);
        
        assert_eq!(processo.recursos_necessarios.get(&Recurso::Impressora), Some(&1));
        assert_eq!(processo.recursos_necessarios.get(&Recurso::Memoria(256)), Some(&2));
        assert_eq!(processo.recursos_necessarios.len(), 2);
    }

    #[test]
    fn test_nucleo_new() {
        let nucleo = Nucleo::new(0);
        
        assert_eq!(nucleo.id, 0);
        assert!(nucleo.processo_atual.is_none());
        assert_eq!(nucleo.tempo_ocioso, 0);
    }

    #[test]
    fn test_sistema_new() {
        let sistema = Sistema::new(4, 10, AlgoritmoEscalonamento::RoundRobin);
        
        assert_eq!(sistema.nucleos.len(), 4);
        assert_eq!(sistema.quantum, 10);
        assert_eq!(sistema.algoritmo, AlgoritmoEscalonamento::RoundRobin);
        assert_eq!(sistema.tempo_global, 0);
        assert_eq!(sistema.taxa_chegada_processos, 20);
        assert!(sistema.processos.is_empty());
        assert!(sistema.processos_bloqueados.is_empty());
        
        // Verificar recursos iniciais
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Impressora), Some(&2));
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Scanner), Some(&1));
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Disco), Some(&3));
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Memoria(1024)), Some(&8));
    }

    #[test]
    fn test_adicionar_processo() {
        let mut sistema = Sistema::new(2, 5, AlgoritmoEscalonamento::RoundRobin);
        let processo = Processo::new(1, 50, 3);
        
        sistema.adicionar_processo(processo);
        
        assert_eq!(sistema.processos.len(), 1);
        assert_eq!(sistema.processos[0].id, 1);
        assert_eq!(sistema.processos[0].estado, EstadoProcesso::Pronto);
    }

    #[test]
    fn test_escolher_proximo_processo_round_robin() {
        let mut sistema = Sistema::new(1, 5, AlgoritmoEscalonamento::RoundRobin);
        
        let processo1 = Processo::new(1, 30, 1);
        let processo2 = Processo::new(2, 20, 5);
        let processo3 = Processo::new(3, 40, 3);
        
        sistema.adicionar_processo(processo1);
        sistema.adicionar_processo(processo2);
        sistema.adicionar_processo(processo3);
        
        let proximo = sistema.escolher_proximo_processo();
        assert_eq!(proximo.unwrap().id, 1); // Primeiro da fila
    }

    #[test]
    fn test_escolher_proximo_processo_prioridade() {
        let mut sistema = Sistema::new(1, 5, AlgoritmoEscalonamento::Prioridade);
        
        let processo1 = Processo::new(1, 30, 1);
        let processo2 = Processo::new(2, 20, 5);
        let processo3 = Processo::new(3, 40, 3);
        
        sistema.adicionar_processo(processo1);
        sistema.adicionar_processo(processo2);
        sistema.adicionar_processo(processo3);
        
        let proximo = sistema.escolher_proximo_processo();
        assert_eq!(proximo.unwrap().id, 2); // Maior prioridade (5)
    }

    #[test]
    fn test_escolher_proximo_processo_shortest_job_first() {
        let mut sistema = Sistema::new(1, 5, AlgoritmoEscalonamento::ShortestJobFirst);
        
        let processo1 = Processo::new(1, 30, 1);
        let processo2 = Processo::new(2, 20, 5);
        let processo3 = Processo::new(3, 40, 3);
        
        sistema.adicionar_processo(processo1);
        sistema.adicionar_processo(processo2);
        sistema.adicionar_processo(processo3);
        
        let proximo = sistema.escolher_proximo_processo();
        assert_eq!(proximo.unwrap().id, 2); // Menor tempo restante (20)
    }

    #[test]
    fn test_pode_alocar_recursos_insuficientes() {
        let sistema = Sistema::new(2, 5, AlgoritmoEscalonamento::RoundRobin);
        let processo = Processo::new(1, 50, 3)
            .necessita_recurso(Recurso::Impressora, 3) // Só tem 2 disponíveis
            .necessita_recurso(Recurso::Memoria(512), 4);
        
        assert!(!sistema.pode_alocar_recursos(&processo));
    }

    #[test]
    fn test_alocar_recursos_sucesso() {
        let mut sistema = Sistema::new(2, 5, AlgoritmoEscalonamento::RoundRobin);
        let processo = Processo::new(1, 50, 3)
            .necessita_recurso(Recurso::Impressora, 1)
            .necessita_recurso(Recurso::Memoria(1024), 2);
        
        let resultado = sistema.alocar_recursos(&processo);
        assert!(resultado);
        
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Impressora), Some(&1));
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Memoria(1024)), Some(&6));
    }

    #[test]
    fn test_alocar_recursos_falha() {
        let mut sistema = Sistema::new(2, 5, AlgoritmoEscalonamento::RoundRobin);
        let processo = Processo::new(1, 50, 3)
            .necessita_recurso(Recurso::Impressora, 5) // Mais que disponível
            .necessita_recurso(Recurso::Memoria(1024), 2);
        
        let resultado = sistema.alocar_recursos(&processo);
        assert!(!resultado);
        
        // Recursos devem permanecer inalterados
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Impressora), Some(&2));
        assert_eq!(sistema.recursos_disponiveis.get(&Recurso::Memoria(1024)), Some(&8));
    }

    #[test]
    fn test_escalonar_multiplos_processos() {
        let mut sistema = Sistema::new(2, 5, AlgoritmoEscalonamento::RoundRobin);
        
        let processo1 = Processo::new(1, 3, 1);
        let processo2 = Processo::new(2, 2, 2);
        let processo3 = Processo::new(3, 4, 3);
        
        sistema.adicionar_processo(processo1);
        sistema.adicionar_processo(processo2);
        sistema.adicionar_processo(processo3);
        
        // Primeiro passo: dois processos devem ser atribuídos aos dois núcleos
        sistema.escalonar();
        
        let nucleos_ocupados = sistema.nucleos.iter()
            .filter(|n| n.processo_atual.is_some())
            .count();
        assert_eq!(nucleos_ocupados, 2);
        assert_eq!(sistema.processos.len(), 1); // Um processo ainda na fila
    }

    #[test]
    fn test_preempcao_round_robin() {
        let mut sistema = Sistema::new(1, 2, AlgoritmoEscalonamento::RoundRobin); // Quantum 2
        
        let processo1 = Processo::new(1, 5, 1);
        let processo2 = Processo::new(2, 3, 2);
        
        sistema.adicionar_processo(processo1);
        sistema.adicionar_processo(processo2);
        
        // Primeiro processo executa
        sistema.escalonar();
        assert_eq!(sistema.nucleos[0].processo_atual.as_ref().unwrap().id, 1);
        
        // Segundo passo - ainda processo 1
        sistema.escalonar();
        assert_eq!(sistema.nucleos[0].processo_atual.as_ref().unwrap().id, 1);
        
        // Terceiro passo - deve haver preempção (quantum = 2)
        sistema.escalonar();
        assert_eq!(sistema.nucleos[0].processo_atual.as_ref().unwrap().id, 2);
    }

}