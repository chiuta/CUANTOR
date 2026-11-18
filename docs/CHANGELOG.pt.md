# Registo de alterações

## 0.2.0

### Adicionado

- Interface em 8 idiomas (GUI com seletor de idioma, ajuda e erros da CLI).
- Bateria de testes adversariais: fuzzing do analisador, bombas de recursos, verificações cruzadas entre motores, estatística, respostas REST hostis.
- Integração com o IBM Quantum: transpilador para portas nativas com encaminhamento, cliente REST Runtime, exportação para o Composer.
- Site de apresentação e documentação em 8 idiomas.

### Corrigido

- A expansão exponencial de portas definidas pelo utilizador podia reservar gigabytes e abortar o processo; a expansão passou a ser limitada.
- Expressões muito aninhadas provocavam um estouro da pilha e abortavam o processo; a profundidade está limitada a 128.
- Os resultados da amostragem dependiam do número de threads; passam a depender apenas da semente.
- A GUI iniciava um número ilimitado de threads de análise com edições rápidas (memória esgotada com 4096 qubits); agora há um thread de trabalho e um pedido em fila.
- Probabilidades de ruído inválidas (acima de 1, negativas, NaN) eram aceites em silêncio; agora são rejeitadas.
- A CLI ignorava em silêncio opções numéricas inválidas como --shots abc e motores desconhecidos; agora são erros.
- Vários símbolos faltavam no tipo de letra predefinido e surgiam como quadrados vazios; foram substituídos por etiquetas de texto.

### Notas

- Não verificado em hardware IBM real.
- Os circuitos dinâmicos não podem ser desenhados no editor nem enviados para hardware IBM.
- Os títulos dos exemplos continuam em romeno.
