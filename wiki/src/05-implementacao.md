# Implementação e Arquitetura

## Arquitetura do Projeto

O projeto foi organizado em módulos para garantir uma clara separação de responsabilidades, tornando o código mais fácil de entender, manter e testar. Cada arquivo tem um papel bem definido:

### `main.rs`

É o ponto de entrada e a camada de interface com o usuário (UI). Ele é responsável por:

- Exibir o menu e capturar as configurações iniciais do sistema (núcleos, algoritmo).
- Gerenciar o laço principal da simulação interativa (main loop).
- Processar os comandos do usuário em tempo real (pausar, mostrar estatísticas, adicionar processo).

### `sistema.rs`

É o coração do simulador, contendo toda a lógica de negócio e as regras do sistema operacional simulado. Suas responsabilidades incluem:

- Definir as estruturas de dados centrais: `Sistema`, `Processo`, `Nucleo`, e os enums `Recurso` e `EstadoProcesso`.
- Implementar a lógica do escalonador (`escalonar`), que move os processos entre os estados, aloca-os aos núcleos e gerencia o tempo.
- Controlar a alocação e liberação de recursos e a detecção de deadlock.

### `examples.rs`

Este módulo serve como uma biblioteca de cenários de teste pré-configurados. Sua função é facilitar a demonstração e a verificação de diferentes comportamentos do sistema, como:

- `exemplo_round_robin()`: Cria um sistema com processos para testar o algoritmo Round Robin.
- `exemplo_deadlock()`: Configura um cenário específico com recursos limitados para forçar e testar a detecção de deadlock.
- `exemplo_complexo()`: Monta um ambiente com múltiplos processos que demandam diferentes tipos de recursos.

---

## Construtores da Linguagem Usados

Construtores de linguagem são as ferramentas e os blocos de construção fundamentais que uma linguagem oferece para expressar lógica e estruturar dados. Eles incluem desde a forma como definimos tipos (como `structs`) até como controlamos o fluxo de um programa (`match`, `if`, etc).

Neste projeto, utilizamos diversos construtores poderosos do Rust para escrever um código seguro, expressivo e eficiente.

### Structs e Enums

Esses são os pilares para a modelagem de dados do nosso domínio.

#### Structs

Permitem agrupar diferentes dados em uma única unidade lógica. A struct `Processo` é um exemplo perfeito, encapsulando todas as informações de um processo.

```rust
// Em sistema.rs
#[derive(Debug, Clone)]
pub struct Processo {
    pub id: u32,
    pub prioridade: i32,
    pub tempo_total: u32,
    pub tempo_restante: u32,
    // ... outros campos
}
```

#### Enums

Permitem definir um tipo que pode ser um de vários valores possíveis. O `AlgoritmoEscalonamento` garante que apenas algoritmos válidos possam ser usados. Além disso, Rust permite que enums contenham dados, como visto em `Recurso`, tornando o tipo ainda mais poderoso.

```rust
// Em sistema.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Recurso {
    Impressora,
    Scanner,
    Disco,
    Memoria(u32), // Este enum contém um dado!
}
```

---

### Ownership e Borrowing

Este é o sistema que garante a segurança de memória em Rust.

- **Ownership (Posse):** Cada valor em Rust tem uma "dona". Quando um processo é criado e adicionado ao sistema, a posse dele é movida para a fila de processos, garantindo que haja apenas um responsável por aquele dado.
- **Borrowing (Empréstimo):** Para usar um dado sem tomar posse, nós o "emprestamos". A função `escalonar` pega emprestado uma referência mutável ao `Sistema` (`&mut self`), permitindo que ela modifique o sistema de forma segura.

```rust
// Em sistema.rs
impl Sistema {
    pub fn escalonar(&mut self) {
        // O uso de `&mut self` permite que este método
        // modifique os campos do Sistema (processos, núcleos, etc).
    }
}
```

---

### Pattern Matching (`match`)

É uma forma poderosa de controlar o fluxo do programa com base em padrões. O `match` é exaustivo, o que significa que o compilador nos obriga a tratar todas as possibilidades, prevenindo bugs.

```rust
// Em main.rs
fn escolher_algoritmo() -> AlgoritmoEscalonamento {
    loop {
        match escolha {
            1 => return AlgoritmoEscalonamento::RoundRobin,
            2 => return AlgoritmoEscalonamento::Prioridade,
            3 => return AlgoritmoEscalonamento::ShortestJobFirst,
            _ => println!("Opção inválida!"), // Trata todos os outros casos
        }
    }
}
```

---

### Coleções Padrão

Rust oferece um conjunto rico de estruturas de dados na sua biblioteca padrão.

- `VecDeque`: Uma fila de duas pontas, usada aqui como uma fila FIFO (First-In, First-Out) para a nossa fila de processos prontos.
- `HashMap`: Usado para armazenar os recursos disponíveis, mapeando cada `Recurso` para a sua quantidade (`u32`).

```rust
// Em sistema.rs, na struct Sistema
pub struct Sistema {
    pub processos: VecDeque<Processo>,
    pub recursos_disponiveis: HashMap<Recurso, u32>,
    // ...
}
```

---

### Traits

Traits são como interfaces em outras linguagens. Eles definem um conjunto de métodos que um tipo deve implementar para ter um determinado comportamento. Usamos traits da biblioteca padrão para estender nossos tipos.

```rust
// Em sistema.rs
use std::fmt;

impl fmt::Display for AlgoritmoEscalonamento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlgoritmoEscalonamento::RoundRobin => write!(f, "Round Robin"),
            // ...
        }
    }
}
```

---

### Gerenciamento de Erros com `Result` e `Option`

Rust não usa exceções. Em vez disso, operações que podem falhar retornam um enum `Result<T, E>` (sucesso ou erro) ou `Option<T>` (um valor ou nenhum).

No nosso código, a função de leitura lida com a possibilidade de o usuário digitar algo inválido. A função `parse()` retorna um `Result`, e tratamos isso com `if let`.

```rust
// Em main.rs
if let Ok(valor) = input.parse::<usize>() {
    if valor >= min && valor <= max {
        return valor;
    }
}
// Se o parse falhar (retornar Err), o bloco é ignorado
```
