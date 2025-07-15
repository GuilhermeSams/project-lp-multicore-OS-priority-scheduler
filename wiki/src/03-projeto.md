# Projeto: Domínio, Premissas e Usuário

### Domínio:

O projeto está inserido no domínio de Sistemas Operacionais, especificamente na área de Gerenciamento de Processos e Recursos. Ele simula um escalonador que decide qual processo deve ser executado em um conjunto de núcleos de CPU, como e quando alocar recursos (memória, disco, etc.) e como tratar situações de bloqueio e preempção.

### Premissas:

Para manter o escopo gerenciável, o simulador opera sob as seguintes premissas:

O tempo é simulado em unidades discretas (`tempo_global`).

A concorrência é simulada: múltiplos núcleos operam em paralelo, mas a simulação em si é executada em uma única thread principal que gerencia o estado de todos os núcleos.

Os processos são gerados automaticamente a uma taxa configurável ou adicionados manualmente pelo usuário.

Os recursos são abstratos (ex: `Recurso::Impressora`) e sua alocação é binária (ou o processo obtém todos os recursos que precisa ou é bloqueado).

### Usuário:

O usuário alvo deste projeto é um estudante ou entusiasta de Sistemas Operacionais que deseja visualizar e entender o impacto de diferentes algoritmos de escalonamento. A interface interativa via linha de comando foi projetada para ser simples e informativa, permitindo configurar, executar, pausar e inspecionar o estado do sistema em tempo real.
