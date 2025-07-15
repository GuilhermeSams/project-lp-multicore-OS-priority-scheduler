# Adequação do Projeto à Linguagem

1. Gerenciamento Explícito de Recursos e Memória
   Nosso simulador gerencia um conjunto finito de recursos, como Impressora, Scanner e Memoria. A lógica de alocar e liberar esses recursos para os processos é um pilar central do sistema. O modelo de posse (ownership) do Rust, que dispensa um garbage collector, nos dá controle total e determinístico sobre o ciclo de vida dos recursos. Isso espelha com precisão como um sistema operacional real deve operar: com eficiência e previsibilidade, sem pausas inesperadas para limpeza de memória.

2. Garantia de Correção e Estado Consistente
   Um processo transita por estados críticos (Pronto, Executando, Bloqueado). Um bug nessa lógica poderia corromper todo o sistema. O sistema de tipos forte de Rust, combinado com enums e o pattern matching exaustivo do match, nos força a tratar todos os estados possíveis em tempo de compilação. Isso previne uma classe inteira de bugs lógicos, garantindo que a máquina de estados do nosso escalonador seja robusta e correta por design.

3. Segurança em Concorrência por Padrão
   O projeto simula múltiplos núcleos de CPU operando sobre uma fila de processos compartilhada. Em outras linguagens, isso seria um convite para data races (condições de corrida). Em Rust, o verificador de empréstimo (borrow checker) proíbe o acesso simultâneo e inseguro a dados compartilhados em tempo de compilação. Mesmo em uma simulação, essa garantia nos permite escrever uma lógica de concorrência sem medo, sabendo que a principal fonte de bugs em sistemas paralelos já foi eliminada pela própria linguagem.

4. Desempenho com Abstrações de Alto NívelS
   Escalonadores de processos são componentes críticos de desempenho em um sistema. Rust nos permite escrever código de alto nível e expressivo — como usar iteradores para encontrar o próximo processo em escolher_proximo_processo() — com a garantia de que essas abstrações serão compiladas para um código de máquina extremamente rápido, comparável ao de C++. São as famosas abstrações de custo zero, que nos permitem escrever um código legível sem sacrificar a performance necessária para o domínio do problema.

Em suma, o nosso escalonador é um exemplo prático e direto dos problemas que Rust foi criado para resolver. Os desafios de gerenciamento de memória, estado, concorrência e desempenho são centrais tanto para o projeto quanto para a filosofia da linguagem.
