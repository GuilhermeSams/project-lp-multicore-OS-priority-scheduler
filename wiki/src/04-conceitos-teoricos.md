# Fundamentos de Escalonamento de Processos

Para entender o funcionamento do nosso simulador, é crucial primeiro compreender os conceitos teóricos de sistemas operacionais que ele representa. Esta seção serve como um embasamento para a arquitetura e as decisões de implementação do projeto.

### O que é um Processo?

De forma simples, um processo é um programa em execução. Quando você abre um aplicativo, o sistema operacional cria um ou mais processos para executar o código daquele programa. Ele é a unidade fundamental de trabalho em um sistema operacional moderno.

Para gerenciar cada processo, o sistema operacional mantém uma estrutura de dados chamada Bloco de Controle de Processo (PCB). O PCB armazena todas as informações vitais sobre o processo, como seu ID, prioridade, estado atual, contadores, e os recursos que ele está utilizando.

- Conexão com o Projeto: No nosso simulador, a struct Processo é a nossa versão simplificada de um PCB. Ela armazena os campos essenciais que nosso escalonador precisa para tomar suas decisões:

```
pub struct Processo {
    pub id: u32,
    pub prioridade: i32,
    pub tempo_restante: u32,
    pub estado: EstadoProcesso,
    pub recursos_necessarios: HashMap<Recurso, u32>,
}
```

### Os Estados de um Processo

Um processo muda de estado ao longo de seu ciclo de vida. Os estados principais são:

- Pronto (Ready): O processo está pronto para ser executado e aguarda na fila de processos prontos até que o escalonador o escolha para usar a CPU. No nosso código, são os processos na fila `sistema.processos`.

- Executando (Running): O processo está atualmente executando suas instruções em um núcleo de CPU. No nosso código, é um processo que foi atribuído a uma `struct Nucleo`.

- Bloqueado (Blocked): O processo não pode continuar executando porque está esperando por algum evento externo, como a liberação de um recurso (ex: impressora) ou uma operação de E/S. No nosso código, são os processos na fila `sistema.processos_bloqueados`.

- Concluído (Terminated): O processo terminou sua execução e seus recursos são liberados pelo sistema.

O fluxo típico é uma dança entre esses estados, gerenciada pelo escalonador.

#### Pronto ➡️ Executando ➡️ Bloqueado ➡️ Pronto

### O Escalonador (Scheduler)

O escalonador de CPU é o componente do sistema operacional responsável por decidir qual dos processos na fila de `Pronto` será o próximo a receber um núcleo de CPU. Essa decisão é a essência do gerenciamento de processos e é baseada em um algoritmo de escalonamento.

### Critérios de Escalonamento

Não existe um "melhor" algoritmo de escalonamento; cada um otimiza para diferentes objetivos. Os principais critérios para avaliar um algoritmo são:

- Utilização da CPU: Manter a CPU o mais ocupada possível.

- Vazão (Throughput): O número de processos concluídos por unidade de tempo.

- Tempo de Resposta (Response Time): O tempo desde a submissão de um processo até ele começar a executar e produzir a primeira resposta.

- Tempo de Espera (Waiting Time): O tempo total que um processo passa na fila de `Pronto`.

- Justiça (Fairness): Garantir que todo processo receba uma porção justa de tempo de CPU, evitando que algum processo fique indefinidamente esperando (starvation).

### Preempção: A Regra do Jogo

Os algoritmos de escalonamento se dividem em duas grandes categorias:

- Não-Preemptivo: Uma vez que um processo recebe a CPU, ele a mantém até que termine sua execução ou a libere voluntariamente (por exemplo, ao solicitar um recurso e ficar bloqueado).

- Preemptivo: O sistema operacional pode forçar a retirada da CPU de um processo em execução para dá-la a outro, geralmente de maior prioridade. Isso é o que permite a multitarefa fluida em sistemas modernos. Pense nisso como um timer: quando o tempo acaba, o próximo da fila assume, independentemente de o anterior ter terminado.

Os algoritmos implementados em nosso simulador são preemptivos:

- Round Robin: É o exemplo clássico de preempção baseada em tempo (quantum).

- Prioridade e Shortest Job First (SJF): Em nossa implementação, eles são preemptivos. Se um novo processo mais prioritário (ou mais curto, no caso do SJF) chega à fila de `Pronto`, ele pode tomar o lugar de um processo que está atualmente em execução.
