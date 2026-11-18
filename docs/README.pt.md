# CUANTOR

> Um simulador de computador quântico para o seu ambiente de trabalho: de 5 a 1000 qubits, com uma interface ao estilo do IBM Quantum Composer.

**É uma simulação exata que corre no seu próprio computador, não hardware quântico.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## O que faz

- Editor ao estilo Composer: paleta de portas coloridas, fios q[0..n], mostradores de medição por qubit, histograma de probabilidades, esfera Q com anel de fase e código OpenQASM 3.0 / 2.0 / Qiskit em tempo real.
- Três motores exatos: vetor de estado (até 28 qubits), estabilizador para circuitos Clifford (até 4096 qubits) e amplitudes esparsas para circuitos com poucos ramos.
- Circuitos dinâmicos (medição a meio do circuito, reset, if), portas definidas pelo utilizador, ruído de despolarização e de leitura, sementes reproduzíveis.
- Importação de OpenQASM 2.0 e 3.0, exportação para OpenQASM 3.0 e Qiskit.
- Ligação ao IBM Quantum: abra circuitos no IBM Composer ou execute-os em hardware IBM real através da API REST Runtime, com a sua própria chave.
- Interface em 8 idiomas: romeno, inglês, francês, alemão, espanhol, italiano, português, húngaro.
- Windows, macOS e Linux; um único workspace Rust: núcleo, GUI, CLI, cliente IBM.

## Limites físicos

Um estado geral de n qubits exige 2^n amplitudes: 24 qubits ocupam cerca de 256 MiB e 28 qubits cerca de 4 GiB. Além disso só funcionam circuitos com estrutura (Clifford, estados do tipo GHZ, poucos ramos de sobreposição). Um GHZ de 1000 qubits executa-se instantaneamente; um circuito arbitrário de 1000 qubits é matematicamente inacessível a qualquer máquina clássica.

## Início rápido

Compile a partir do código-fonte com uma cadeia de ferramentas Rust atual:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: o menu Ficheiro → «Abrir no IBM Composer» guarda um ficheiro .qasm; no Composer use File → Upload .qasm file.
- Hardware real: defina localmente IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (ou ~/.config/cuantor/ibm.json) e execute `cuantor ibm run circuit.qasm --backend NOME`. A chave nunca é mostrada, registada nem enviada para outro lado que não a IBM.

> Não verificado em hardware real: o transpilador (portas nativas rz/sx/x + cx/cz/ecr, encaminhamento SWAP) é verificado com o simulador integrado, e o cliente REST apenas contra um servidor simulado. Os detalhes dos pedidos podem exigir ajustes; os circuitos dinâmicos não são suportados por este caminho.

## Qualidade e auditoria

A versão 0.2.1 passou por uma auditoria adversarial: 59 testes automáticos, incluindo fuzzing do analisador, bombas de recursos (memória, recursão, aninhamento), ângulos NaN, verificações cruzadas entre os três motores, testes de qui-quadrado da amostragem, reprodutibilidade por semente e respostas REST hostis. A auditoria encontrou e corrigiu 7 defeitos (veja o registo de alterações).

[Registo de alterações](CHANGELOG.pt.md)

## Transferência

Cada versão inclui binários para Windows, macOS (Apple silicon e Intel) e Linux.

## Licença

Ainda não foi definida uma licença (escolha do autor).

---
Criado com Claude. Apenas simulação: não se reivindica qualquer vantagem quântica.
