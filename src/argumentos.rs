//! Este módulo contiene la lógica de validación de argumentos de ejecución.

// Imports de crates externas.
use std::env;

// Constantes.
const CANTIDAD_ARGUMENTOS_ESPERADA: usize = 4;
const POS_RUTA: usize = 1;
const POS_CANT_HILOS: usize = 2;
const POS_NOM_SALIDA: usize = 3;
const EXTENSION_ARCHIVO_SALIDA: &str = ".json";

// Mensajes.
const ERROR_USO_INCORRECTO: &str = "⚠️ Uso incorrecto.";
const EXPLICACION_DE_EJECUCION: &str =
    "👉 Ejecutá el programa como: cargo run <ruta-archivo> <cantidad-threads> <archivo-salida>";
const ERROR_TIPO_CANT_HILOS: &str = "❌ El valor de hilos debe ser un número entero positivo.";
const ERROR_CANTIDAD_HILOS: &str = "❌ Demasiados hilos solicitados: pediste";
const EXPLICACION_HILOS_1: &str = "Tu máquina tiene";
const EXPLICACION_HILOS_2: &str = "CPUs lógicos, por lo tanto el máximo permitido es";
const EXPLICACION_HILOS_3: &str = "hilos (10x).";

/// Se eligio este valor de 'Multiplicador' para permitir el sobredimensionamiento de la concurrencia
/// (para evaluar comportamiento borde) pero sin ser valores innecesariamente incoherentes.
const MULTIPLICADOR_CANT_HILOS: usize = 10;

/// Estructura auxiliar para guardar la configuracion seleccionada al ejecutar el programa.
pub struct Configuracion {
    pub ruta_archivo: String,
    pub cantidad_threads: usize,
    pub nombre_archivo_salida: String,
}

/// Funcion que valida la cantidad de argumentos recibidos por consola.  
/// Devuelve los argumentos en formato 'Vector de Strings'.
fn obtener_argumentos() -> Option<Vec<String>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != CANTIDAD_ARGUMENTOS_ESPERADA {
        eprintln!("{}", ERROR_USO_INCORRECTO);
        eprintln!("{}", EXPLICACION_DE_EJECUCION);
        return None;
    }

    Some(args)
}

/// Funcion que valida la cantidad de hilos ingresados por consola.  
/// Devuelve la cantidad de hilos ingresada y validada.
fn validar_cantidad_hilos(hilos_str: &str) -> Option<usize> {
    // Valida que sea un numero entero positivo.
    let cantidad_threads = match hilos_str.parse::<usize>() {
        Ok(valor) => valor,
        Err(_) => {
            eprintln!("{}", ERROR_TIPO_CANT_HILOS);
            return None;
        }
    };

    // Valida cantidades coherentes de hilos en base al "CPU" del usuario.
    let hilos_disponibles = num_cpus::get();
    let hilos_maximos = hilos_disponibles * MULTIPLICADOR_CANT_HILOS;
    if cantidad_threads > hilos_maximos {
        eprintln!("{} {}.", ERROR_CANTIDAD_HILOS, cantidad_threads);
        eprintln!(
            "{} {} {} {} {}",
            EXPLICACION_HILOS_1,
            hilos_disponibles,
            EXPLICACION_HILOS_2,
            hilos_maximos,
            EXPLICACION_HILOS_3,
        );

        return None;
    }

    Some(cantidad_threads)
}

/// Parsea, valida y almacena los argumentos recibidos por consola.  
/// Devuelve la 'Configuracion' segun los argumentos recibidos en la ejecución del programa.
pub fn parsear_argumentos() -> Option<Configuracion> {
    let args = obtener_argumentos()?;
    let ruta_archivo = args[POS_RUTA].clone();
    let cantidad_threads = validar_cantidad_hilos(&args[POS_CANT_HILOS])?;
    let mut nombre_archivo_salida = args[POS_NOM_SALIDA].clone();
    if !nombre_archivo_salida.ends_with(EXTENSION_ARCHIVO_SALIDA) {
        nombre_archivo_salida.push_str(EXTENSION_ARCHIVO_SALIDA);
    }

    Some(Configuracion {
        ruta_archivo,
        cantidad_threads,
        nombre_archivo_salida,
    })
}
