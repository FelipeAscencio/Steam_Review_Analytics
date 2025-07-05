# Trabajo Práctico Individual "Fork Join" 

## Materia: Programación Concurrente.

### Alumno: Ascencio Felipe Santino.

### Padrón: 110675.

### Video explicativo.

[Link al video](https://drive.google.com/drive/folders/1AwMEUB41e1ugcRlc8IgNJUbbCyO33q3h?usp=drive_link)

### Ejecución del programa

```
cargo run <input-path> <num-threads> <output-file-name>
```

Por ejemplo:

```
cargo run dataset 4 output.json
```

o

```
cargo run dataset 4 output
```

Si el usuario no especifica el formato de salida ".json", el programa lo añade solo. Pero, en caso contrario, el programa no hace el añadido para evitar archivos de salida del tipo "output.json.json".

#### release

Se recomienda fuertemente (para mejoras de performance), ejecutar con el flag '--release', como se muestra a continuación:

```
cargo run --release dataset 4 output.json
```

### Ejecución de los tests creados

```
cargo test -- --nocapture
```

#### Explicación de los tests creados

Los tests que se implementaron verifican que el programa desarrollado para la resolución del trabajo práctico sea 'determinístico'.

En los mismos se prueba que se obtengan los mismos resultados en varios escenarios de ejecución con 2 enfoques distintos:
- Se prueba procesar el archivo con distintas cantidades de hilos, verificando obtener siempre el mismo resultado.
- Se prueba varias veces procesar el archivo con una misma cantidad de hilos, verificando obtener siempre el mismo resultado.

### Crates externos utilizados

- csv (Para parsear y manejar mas facil los datasets).
- serde/serde_json (Para crear los archivos de salida '.json').
- rayon (Para todas las funciones relacionadas a la concurrencia, principalmente el manejo de la 'pool de threads'. Se eligió esta 'crate' porque implementa el 'Worker Stealer' que mejora el rendimiento repartiendo mejor las tareas entre los hilos 'Trabajadores').
- num_cpus (Se usa para, en base a los procesadores del usuario, poner un límite arbitrario que restringa la entrada del mismo en la ejecución del programa a valores coherentes de cantidades de hilos según su computador).

### Explicación de directorios

#### src

En este directorio se encuentra todo el código desarrollado para el funcionamiento del trabajo práctico, exceptuando el "Cargo.toml" que se encuentra en el directorio raíz tal y como lo pide el enunciado.

El programa desarrollado se encuentra dividido en diversos módulos (donde cada uno contiene la lógica y/o estructura/s específica/s para una tarea determinada), estos mismos están documentados siguiendo el estándar "cargo doc", además están formateados con "cargo fmt" y no arrojan ninguna alerta al correr "clippy".

#### dataset

En este directorio se encuentra el dataset auxiliar (steam_short_reviews.csv) utilizado para el desarrollo del proyecto (Esto a fin de, en un inicio, desarrollar y comprobar el funcionamiento con volumenes de datos mucho mas pequeños).

Por limitaciones de 'GitHub' no se puede subir el "dataset" completo brindado por el enunciado, pero de querer trabajar con el mismo se anexa a continuación el link de obtención oficial.

[Link oficial al dataset](https://www.kaggle.com/datasets/najzeko/steam-reviews-2021)

#### dataset_test

En este directorio se encuentran los datasets utilizados para las pruebas automáticas (Los 10 datasets contienen exactamente la misma información que "steam_short_reviews.csv").

Se dividió en varios archivos el contenido de los tests para asegurarse que el hilo 'Productor' genera varios 'chunks' para ser procesados por los hilos 'Trabajadores', sin necesidad de tener archivos extremadamente grandes (esto porque al terminar de leer un archivo, se termina un chunk y se envía, sin necesidad de tener el tamaño máximo por chunk para ser enviado).

#### output

En este directorio se guardan los ".json" generados como resultado del análisis de los 'datasets'.

- "short_reviews_result.json": Contiene el resultado del análisis del archivo "steam_short_reviews.csv" que se utilizó para el desarrollo inicial del programa.
- "output_generado.json": Contiene el resultado del análisis del dataset del enunciado, donde solo se aprecian 2 diferencias con el "expected_output.json":
  - El 'Padrón' generado en el reporte es el mío (110675), en reemplazo al que figura como ejemplo.
  - La cantidad de 'Reviews' en inglés tiene una menos (9635436 en vez de 9635437) ya que, como se indicó por los profesores, las reviews con "votes_helpful" que no pueden ser almacenadas en 'u32' deben ser ignoradas (hay un caso solo en el dataset de ejemplo), para que no figuren en el reporte final. Como decisión de diseño decidí tampoco contar la review para mantener la fidelidad del análisis generado simplemente ignorando en su totalidad las reviews con una cantidad de "votes_helpful" que no pueden ser almacenadas en un 'u32'.

### Conclusiones de rendimiento

Una vez finalizado el programa, se realizaron mediciones de rendimiento con distintas cantidades de hilos, para verificar que se obtenga una mejora del rendimiento (ejecutando la versión 'release').

Si bien este rendimiento es totalmente variable por la computadora del usuario, sirvió para verificar que la implementación realizada muestra mejoras en 'perfomance' en entornos multiprocesadores, que es lo que se buscaba con el desarrollo del trabajo práctico.

Las pruebas se realizaron en una computadora de gamma media con "8 cores", "8gb de ram" y los resultados obtenidos fueron:
- 1 Hilo: 30 segundos en finalizar.
- 2 Hilos: 28 segundos en finalizar.
- 4 Hilos: 27 segundos en finalizar.
- 6 Hilos: 26 segundos en finalizar.
- 8 Hilos: 25 segundos en finalizar.

Por lo que se observa que se consiguió una mejora, en rendimiento, de aproximadamente '1 Segundo' por cada par de hilos agregados, lo cual demuestra la mejora de la implementación en entornos multiprocesadores.
