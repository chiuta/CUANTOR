# CUANTOR

> Um simulador de computador quântico para o seu ambiente de trabalho: de 5 a 1000 qubits, com uma interface ao estilo do IBM Quantum Composer.

**É um computador quântico de faz de conta dentro do teu computador: matemática exata, mas não hardware quântico real.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — numa frase:** O CUANTOR é um computador quântico de brincar no teu ecrã. Clicas em blocos coloridos, carregas em Executar e vês a resposta. É de faz de conta, não o verdadeiro.

## O que faz

- Um quadro de desenho como o Composer da IBM: escolhes blocos coloridos (portas), pões-nos em linhas (qubits) e barras e uma bola mostram o que acontece. Um painel de texto escreve em direto o mesmo circuito em OpenQASM e Qiskit.
- Três calculadoras lá dentro: uma que se lembra de todos os números (até 28 qubits), uma rapidíssima para portas simples (até 4096 qubits) e outra para circuitos com poucos ramos.
- Pode decidir pelo caminho (medição, reset, if). Podes inventar portas, juntar erros de faz de conta (ruído) e repetir qualquer resultado com uma semente (seed).
- Lê ficheiros OpenQASM 2.0 e 3.0 e guarda OpenQASM 3.0 ou Qiskit.
- Fala com a IBM Quantum: abre o circuito no IBM Composer ou envia-o para uma máquina IBM real com a chave da tua conta.
- Fala 8 idiomas: romeno, inglês, francês, alemão, espanhol, italiano, português, húngaro.
- Funciona em Windows, macOS e Linux. Lá dentro há um só projeto Rust com quatro partes: núcleo, janela, linha de comandos e ligação à IBM.

> **ELI5 — numa frase:** Blocos coloridos, três calculadoras, 8 idiomas, funciona em qualquer computador.

## Limites físicos

Cada qubit a mais duplica o trabalho, como um caderno em que cada qubit novo duplica as páginas. 24 qubits precisam de cerca de 256 MiB, 28 qubits de cerca de 4 GiB. Daí em diante só funcionam padrões simples. Um grupo de 1000 qubits todos de acordo (GHZ) corre num instante; um circuito qualquer de 1000 qubits é impossível para qualquer computador normal.

> **ELI5 — numa frase:** Mais qubits, muito mais trabalho. Padrões simples podem ser enormes, os baralhados não.

## Início rápido

O mais fácil: descarrega o programa (ver Descarga) e faz duplo clique em cuantor-gui. Para o compilar tu próprio com um Rust recente:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — numa frase:** Descarregas, duplo clique em cuantor-gui e brincas.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: no CUANTOR carrega em «Abrir no IBM Composer». É guardado um ficheiro .qasm. No Composer da IBM escolhe File → Upload .qasm file.
- Uma máquina real: põe a tua chave IBM, a instância (CRN) e o nome da máquina em IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (ou em ~/.config/cuantor/ibm.json) e executa `cuantor ibm run circuit.qasm --backend NOME`. A chave só sai do teu computador em direção à IBM.

> Aviso sincero: esta parte ainda não foi experimentada numa máquina IBM real, só num servidor de faz de conta e com o simulador integrado. Podem ser precisas pequenas correções. Os circuitos com reset ou if não podem ser enviados.

> **ELI5 — numa frase:** Podes enviar o circuito para o site da IBM ou para uma máquina IBM real. A real ainda não experimentámos.

## Qualidade e auditoria

Antes da versão 0.2.1 tentámos por todos os meios estragar o programa: 59 testes automáticos, dados disparatados, circuitos feitos para comer toda a memória, ângulos impossíveis, as três calculadoras comparadas entre si e respostas falsas da IBM que mentem. Encontrámos e corrigimos 7 erros (ver o registo de alterações).

> **ELI5 — numa frase:** Tentámos estragá-lo, encontrámos 7 erros e corrigimo-los.

[Registo de alterações](CHANGELOG.pt.md)

## Transferência

Os programas para Windows, macOS (chip Apple e Intel) e Linux vão anexados a cada versão. Escolhe o teu abaixo.

**[Última versão](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Guia de utilização (PDF)

Explicado como para uma criança de 10 anos, com um mini resumo para uma de 5 anos depois de cada capítulo. O mesmo guia em 8 idiomas.

**[Descarregar o guia (PDF)](guide/CUANTOR-guide.pt.pdf)** · Todos os idiomas: [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — numa frase:** Escolhe o teu computador, descarrega, pronto. O guia é um PDF.

## Licença

Ainda não foi definida uma licença (escolha do autor).

---
Criado com Claude. Apenas simulação: não se reivindica qualquer vantagem quântica.
