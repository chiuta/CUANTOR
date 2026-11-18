# Registro de cambios

## 0.2.0

### Añadido

- Interfaz en 8 idiomas (GUI con selector de idioma, ayuda y errores de la CLI).
- Batería de pruebas adversariales: fuzzing del analizador, bombas de recursos, contrastes entre motores, estadística, respuestas REST hostiles.
- Integración con IBM Quantum: transpilador a puertas nativas con enrutamiento, cliente REST de Runtime, exportación a Composer.
- Sitio web de presentación y documentación en 8 idiomas.

### Corregido

- La expansión exponencial de puertas definidas por el usuario podía reservar gigabytes y abortar el proceso; la expansión ahora está acotada.
- Las expresiones muy anidadas desbordaban la pila y abortaban el proceso; la profundidad se limita a 128.
- Los resultados del muestreo dependían del número de hilos; ahora dependen solo de la semilla.
- La GUI lanzaba un número ilimitado de hilos de análisis con ediciones rápidas (memoria agotada con 4096 cúbits); ahora hay un hilo de trabajo y una solicitud en cola.
- Las probabilidades de ruido no válidas (mayores que 1, negativas, NaN) se aceptaban en silencio; ahora se rechazan.
- La CLI ignoraba en silencio opciones numéricas no válidas como --shots abc y motores desconocidos; ahora son errores.
- Varios símbolos faltaban en la fuente predeterminada y aparecían como recuadros vacíos; se sustituyeron por etiquetas de texto.

### Notas

- No verificado en hardware real de IBM.
- Los circuitos dinámicos no se pueden dibujar en el editor ni enviar al hardware de IBM.
- Los títulos de los ejemplos siguen en rumano.
