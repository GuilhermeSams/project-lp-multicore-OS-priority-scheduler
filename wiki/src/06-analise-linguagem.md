# Análise Aprofundada da Linguagem Rust

Avaliar uma linguagem de programação vai além da simples análise de sua sintaxe. Envolve examinar sua filosofia, pontos fortes e fracos, e como ela se comporta em cenários reais. Para o nosso simulador de escalonamento, um projeto inserido no domínio de sistemas operacionais, a escolha da linguagem Rust se mostra não apenas adequada, mas altamente vantajosa, especialmente quando comparada à sua antecessora, a linguagem C.

---

### 1. Legibilidade (Readability)

A legibilidade refere-se à facilidade com que um ser humano consegue compreender o que um trecho de código realiza. Trata-se de um dos fatores mais críticos para a manutenção a longo prazo de qualquer software.

A sintaxe do Rust é, à primeira vista, mais verbosa do que a do C. Palavras-chave como `fn`, `let mut` e `match` tornam o código mais extenso, mas essa verbosidade é uma escolha deliberada de design em favor da explicitude. Enquanto em C a declaração `int *p;` pode ser ambígua quanto à intenção, em Rust a distinção entre `let p: &i32` (referência imutável) e `let p: &mut i32` (referência mutável) torna o propósito inequívoco.

No projeto, a declaração da `struct Processo` é auto-contida e clara. Em C, a mesma estrutura poderia ter sua definição em um arquivo `.c` e suas declarações espalhadas por arquivos de cabeçalho (`.h`), dificultando a compreensão da abstração como um todo.

Rust também se destaca nas instruções de controle. O `match` representa uma evolução significativa em relação ao `switch` do C, por ser exaustivo (o compilador exige que todos os casos sejam tratados) e por eliminar o perigoso comportamento de _fall-through_, comum fonte de erros. A função `escolher_algoritmo()` é um excelente exemplo de como `match` contribui para uma lógica clara, segura e legível.

O principal ponto de atrito, especialmente para iniciantes, reside na sintaxe de posse e empréstimo (`&`, `&mut`) e nos tempos de vida (`'a`). Contudo, para programadores experientes, esses elementos representam uma camada informacional valiosa, tornando explícitas as regras de gerenciamento de memória que, em C, são meramente convencionais e implícitas.

---

### 2. Capacidade de Escrita (Writability)

A capacidade de escrita refere-se à facilidade com que um programador consegue traduzir uma ideia em código funcional e correto.

Rust oferece mecanismos de abstração superiores aos do C padrão. Os `traits` constituem um sistema de interfaces poderoso que viabiliza o polimorfismo ad-hoc. No projeto, ao implementarmos o `trait Display` para o enum `AlgoritmoEscalonamento`, tornamos possível formatar instâncias desse tipo como strings de forma elegante. Em C, atingir esse mesmo efeito requereria funções auxiliares manuais e menos integradas ao sistema de tipos.

A expressividade do Rust é, possivelmente, seu maior trunfo na capacidade de escrita. A função `escolher_proximo_processo` ilustra bem esse ponto. A lógica "encontre o processo com maior prioridade" é escrita em uma única linha clara:

```rust
self.processos.iter().max_by_key(|p| p.prioridade)
```

Em C, isso exigiria um laço `for`, variáveis auxiliares para armazenar o valor máximo e ponteiros, aumentando a complexidade e o risco de erros.

---

### 3. Confiabilidade (Reliability)

A confiabilidade é a capacidade de um programa funcionar conforme o esperado, sem falhas ou comportamentos indefinidos. Nesse aspecto, Rust diverge profundamente do C, oferecendo um nível de segurança significativamente superior.

O pilar central é a **segurança de memória garantida em tempo de compilação**. Em C, o programador é responsável pelo uso de `malloc()` e `free()`, o que frequentemente leva a erros como _use-after-free_, _double-free_ e _buffer overflows_. Rust elimina completamente essa categoria de falhas. No projeto, quando um `Processo` é inserido no `VecDeque`, sua posse é transferida. Quando esse vetor é descartado, o compilador garante a liberação correta da memória.

Em contraste, em C seria necessário iterar sobre a estrutura e liberar manualmente cada item com `free()`, um processo suscetível a esquecimentos e vazamentos de memória.

Outro aspecto crucial é o tratamento de erros. C tradicionalmente depende de convenções como retornar `-1` ou `NULL`, que podem ser ignoradas. Rust, com os tipos `Result<T, E>` e `Option<T>`, força o programador a tratar explicitamente as falhas. A função `ler_entrada_usize` exemplifica esse comportamento: se o usuário digitar "abc", o programa lida com o erro de forma segura, sem falhas ou comportamentos inesperados.

---

### 4. Custo Total de Propriedade

O custo de uma linguagem deve ser avaliado ao longo de todo o ciclo de vida do software.

- **Custo de treinamento:** Rust possui uma curva de aprendizado mais íngreme que C, especialmente devido ao sistema de posse e empréstimos. Contudo, aprender a evitar armadilhas comuns em C também exige um esforço substancial.
- **Custo de escrita:** O tempo necessário para convencer o compilador de Rust da segurança do código é maior no início, mas evita horas de depuração posteriores.
- **Custo de compilação:** Rust possui um compilador robusto e detalhado, que realiza verificações pesadas. Isso pode resultar em tempos de compilação mais longos, mas com um retorno em confiabilidade.
- **Custo de execução:** O desempenho em tempo de execução é comparável ao de C, frequentemente com resultados excelentes, devido à ausência de _garbage collector_ e à otimização agressiva.
- **Custo de manutenção:** Rust se destaca. A forte tipagem, ausência de _null pointers_, e o sistema de empréstimos impedem classes inteiras de bugs. Alterações no código são mais seguras, reduzindo custos com correções e retrabalho.

---

### 5. Ecossistema e Ferramental (Ecosystem and Tooling)

Esse critério é fundamental em avaliações modernas. Rust oferece um ecossistema altamente produtivo.

- **Cargo:** Gerenciador de pacotes e sistema de build unificado. Com comandos como `cargo run`, `cargo test` e `cargo build`, é possível automatizar tarefas que, em C, exigem `Makefile` ou `CMake`.
- **Clippy:** Ferramenta de análise estática que oferece sugestões de estilo, performance e segurança.
- **rust-analyzer:** Proporciona autocompletar, refatorações e feedback em tempo real nas principais IDEs.
- **cargo doc:** Gera documentação rica automaticamente a partir dos comentários do código, superando ferramentas tradicionais como Doxygen.

Juntas, essas ferramentas elevam a produtividade e profissionalismo do desenvolvimento em Rust, tornando-o uma escolha moderna e segura para projetos de sistemas.

---
