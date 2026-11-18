# CUANTOR

> Un simulador de computador cuántico para tu escritorio: de 5 a 1000 cúbits, con una interfaz al estilo de IBM Quantum Composer.

**Es una simulación exacta que se ejecuta en tu propio ordenador, no hardware cuántico.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## Qué hace

- Editor al estilo Composer: paleta de puertas de colores, hilos q[0..n], diales de medición por cúbit, histograma de probabilidades, esfera Q con anillo de fase y código OpenQASM 3.0 / 2.0 / Qiskit en vivo.
- Tres motores exactos: vector de estado (hasta 28 cúbits), estabilizador para circuitos Clifford (hasta 4096 cúbits) y amplitudes dispersas para circuitos con pocas ramas.
- Circuitos dinámicos (medición intermedia, reset, if), puertas definidas por el usuario, ruido de despolarización y de lectura, semillas reproducibles.
- Importación de OpenQASM 2.0 y 3.0, exportación a OpenQASM 3.0 y Qiskit.
- Enlace con IBM Quantum: abre los circuitos en IBM Composer o ejecútalos en hardware real de IBM mediante la API REST de Runtime, con tu propia clave.
- Interfaz en 8 idiomas: rumano, inglés, francés, alemán, español, italiano, portugués, húngaro.
- Windows, macOS y Linux; un único espacio de trabajo Rust: núcleo, GUI, CLI, cliente de IBM.

## Límites físicos

Un estado general de n cúbits necesita 2^n amplitudes: 24 cúbits ocupan unos 256 MiB y 28 cúbits unos 4 GiB. Más allá solo funcionan los circuitos con estructura (Clifford, estados tipo GHZ, pocas ramas de superposición). Un GHZ de 1000 cúbits se ejecuta al instante; un circuito arbitrario de 1000 cúbits es matemáticamente inalcanzable para cualquier máquina clásica.

## Inicio rápido

Compila desde el código fuente con una cadena de herramientas de Rust actual:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: el menú Archivo → «Abrir en IBM Composer» guarda un archivo .qasm; en Composer usa File → Upload .qasm file.
- Hardware real: define localmente IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (o ~/.config/cuantor/ibm.json) y ejecuta `cuantor ibm run circuit.qasm --backend NOMBRE`. La clave nunca se muestra, se registra ni se envía a otro sitio que no sea IBM.

> No verificado en hardware real: el transpilador (puertas nativas rz/sx/x + cx/cz/ecr, enrutamiento SWAP) se verifica con el simulador integrado, y el cliente REST solo contra un servidor simulado. Los detalles de las solicitudes pueden requerir ajustes; los circuitos dinámicos no se admiten por esta vía.

## Calidad y auditoría

La versión 0.2.0 pasó por una auditoría adversarial: 59 pruebas automáticas, con fuzzing del analizador, bombas de recursos (memoria, recursión, anidamiento), ángulos NaN, contrastes entre los tres motores, pruebas de chi cuadrado del muestreo, reproducibilidad por semilla y respuestas REST hostiles. La auditoría encontró y corrigió 7 defectos (véase el registro de cambios).

[Registro de cambios](CHANGELOG.es.md)

## Descarga

Cada versión incluye binarios para Windows, macOS (Apple silicon e Intel) y Linux.

## Licencia

Todavía no se ha definido una licencia (decisión del autor).

---
Creado con Claude. Solo simulación: no se afirma ninguna ventaja cuántica.
