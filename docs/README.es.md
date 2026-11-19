# CUANTOR

> Un simulador de computador cuántico para tu escritorio: de 5 a 1000 cúbits, con una interfaz al estilo de IBM Quantum Composer.

**Es un ordenador cuántico de mentira dentro de tu ordenador: matemáticas exactas, pero no hardware cuántico real.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — en una frase:** CUANTOR es un ordenador cuántico de juguete en tu pantalla. Pulsas bloques de colores, das a Ejecutar y ves la respuesta. Es de mentira, no el de verdad.

## Qué hace

- Una pizarra como el Composer de IBM: eliges bloques de colores (puertas), los pones en líneas (qubits) y barras y una bola muestran lo que pasa. Un panel de texto escribe en directo el mismo circuito en OpenQASM y Qiskit.
- Tres calculadoras dentro: una que recuerda todos los números (hasta 28 qubits), una rapidísima para puertas simples (hasta 4096 qubits) y otra para circuitos con pocas ramas.
- Puede decidir sobre la marcha (medición, reset, if). Puedes inventar tus puertas, añadir errores de mentira (ruido) y repetir cualquier resultado con una semilla (seed).
- Lee archivos OpenQASM 2.0 y 3.0 y guarda OpenQASM 3.0 o Qiskit.
- Habla con IBM Quantum: abre tu circuito en IBM Composer o envíalo a una máquina IBM real con la clave de tu cuenta.
- Habla 8 idiomas: rumano, inglés, francés, alemán, español, italiano, portugués, húngaro.
- Funciona en Windows, macOS y Linux. Dentro hay un solo proyecto Rust con cuatro partes: núcleo, ventana, línea de comandos y enlace con IBM.

> **ELI5 — en una frase:** Bloques de colores, tres calculadoras, 8 idiomas, funciona en cualquier ordenador.

## Límites físicos

Cada qubit extra duplica el trabajo, como un cuaderno donde cada qubit nuevo duplica las páginas. 24 qubits necesitan unos 256 MiB, 28 qubits unos 4 GiB. Más allá solo funcionan patrones simples. Un grupo de 1000 qubits todos de acuerdo (GHZ) corre al instante; un circuito cualquiera de 1000 qubits es imposible para cualquier ordenador normal.

> **ELI5 — en una frase:** Más qubits, mucho más trabajo. Los patrones simples pueden ser enormes, los enredados no.

## Inicio rápido

Lo más fácil: descarga el programa (ver Descarga) y haz doble clic en cuantor-gui. Para compilarlo tú mismo con un Rust reciente:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — en una frase:** Descargas, doble clic en cuantor-gui y a jugar.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: en CUANTOR pulsa «Abrir en IBM Composer». Se guarda un archivo .qasm. En el Composer de IBM elige File → Upload .qasm file.
- Una máquina real: pon tu clave de IBM, la instancia (CRN) y el nombre de la máquina en IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (o en ~/.config/cuantor/ibm.json) y ejecuta `cuantor ibm run circuit.qasm --backend NOMBRE`. La clave solo sale de tu ordenador hacia IBM.

> Aviso sincero: esta parte aún no se ha probado en una máquina IBM real, solo en un servidor de mentira y con el simulador integrado. Pueden hacer falta pequeños arreglos. Los circuitos con reset o if no se pueden enviar.

> **ELI5 — en una frase:** Puedes enviar tu circuito a la web de IBM o a una máquina IBM real. La máquina real aún no la hemos probado.

## Calidad y auditoría

Antes de la versión 0.2.1 intentamos romper el programa por todos los medios: 59 pruebas automáticas, datos absurdos, circuitos hechos para comerse toda la memoria, ángulos imposibles, las tres calculadoras comparadas entre sí y respuestas falsas de IBM que mienten. Encontramos y arreglamos 7 errores (ver el registro de cambios).

> **ELI5 — en una frase:** Intentamos romperlo, encontramos 7 errores y los arreglamos.

[Registro de cambios](CHANGELOG.es.md)

## Descarga

Los programas para Windows, macOS (chip Apple e Intel) y Linux van adjuntos a cada versión. Elige el tuyo abajo.

**[Última versión](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Guía de uso (PDF)

Explicada como para un niño de 10 años, con un mini resumen para uno de 5 años después de cada capítulo. La misma guía en 8 idiomas.

**[Descargar la guía (PDF)](guide/CUANTOR-guide.es.pdf)** · Todos los idiomas: [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — en una frase:** Elige tu ordenador, descarga y listo. La guía es un PDF.

## Licencia

Todavía no se ha definido una licencia (decisión del autor).

---
Creado con Claude. Solo simulación: no se afirma ninguna ventaja cuántica.
