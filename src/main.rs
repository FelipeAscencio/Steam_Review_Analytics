//! Este módulo contiene la lógica principal/secuencial del programa.

// Módulos locales utilizados.
mod argumentos;
mod estadisticas;
mod estadisticas_serializables;
mod procesadores;
mod reviews_parseadas;

// Módulo local para 'test'.
#[cfg(test)]
mod tests_concurrencia;

// Imports de crates externas.
use rayon::ThreadPoolBuilder;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

// Imports de funciones/estructuras propias.
use crate::estadisticas::EstadisticasGlobales;
use crate::estadisticas_serializables::{ASalidaFinal, SalidaFinal, filtrar_top3};
use argumentos::parsear_argumentos;
use procesadores::procesar_csv_con_rayon;

// Constantes.
const CODIGO_ERROR: i32 = 1;
const PADRON: u32 = 110675;

// Mensajes.
const NOMBRE_DIRECTORIO_OUTPUT: &str = "output";
const ERROR_CREACION_POOL: &str = "❌ No se pudo crear la pool de hilos.";
const ERROR_PROC_ARCHIVO: &str = "❌ Error al procesar el archivo:";
const ERROR_NO_SE_SERIALIZO: &str = "❌ No se pudo serializar a JSON.";
const ERROR_CREACION_CARPETA: &str = "❌ No se pudo crear la carpeta: ";
const ERROR_CREACION_ARCHIVO: &str = "❌ No se pudo crear el archivo: ";
const ERROR_ESCRITURA_JSON: &str = "❌ No se pudo escribir el JSON.";
const MSJ_GUARDADO_RESULTADO: &str = "✅ Estadísticas guardadas en:";

/// Función que controla:
/// - La creación del `thread pool`.
/// - Pasa a procesar el `.csv`.
/// - Hace el `merge` de los resultados.
///
/// Devuelve las estadísticas globales del `.csv` procesado.
fn procesar_archivo_con_pool(ruta: String, cantidad_threads: usize) -> EstadisticasGlobales {
    let pool = ThreadPoolBuilder::new()
        .num_threads(cantidad_threads)
        .build()
        .expect(ERROR_CREACION_POOL);

    let mut conteo_global = EstadisticasGlobales::default();
    pool.install(|| match procesar_csv_con_rayon(ruta) {
        Ok(parciales) => {
            for parcial in parciales {
                parcial.merge_into(&mut conteo_global);
            }
        }

        Err(e) => {
            eprintln!("{} {}.", ERROR_PROC_ARCHIVO, e);
            std::process::exit(CODIGO_ERROR);
        }
    });

    conteo_global
}

/// Función que filtra los resultados obtenidos del archivo procesado.
///
/// Devuelve la estructura lista para ser escrita en formato `.json`.
fn preparar_salida_final(conteo: &EstadisticasGlobales) -> SalidaFinal {
    filtrar_top3(conteo).a_salida_final(PADRON)
}

/// Función que crea el `.json` con el resultado obtenido.
fn guardar_json_de_salida(salida: &SalidaFinal, nombre_archivo: &str) {
    let json = serde_json::to_string_pretty(salida).expect(ERROR_NO_SE_SERIALIZO);

    create_dir_all(NOMBRE_DIRECTORIO_OUTPUT)
        .unwrap_or_else(|_| panic!("{} {}.", ERROR_CREACION_CARPETA, NOMBRE_DIRECTORIO_OUTPUT));

    let ruta_salida = Path::new(NOMBRE_DIRECTORIO_OUTPUT).join(nombre_archivo);
    let mut archivo = File::create(&ruta_salida)
        .unwrap_or_else(|_| panic!("{} {}.", ERROR_CREACION_ARCHIVO, ruta_salida.display()));

    archivo
        .write_all(json.as_bytes())
        .expect(ERROR_ESCRITURA_JSON);

    println!("{} '{}'.", MSJ_GUARDADO_RESULTADO, ruta_salida.display());
}

/// Punto de entrada del programa.
fn main() {
    let configuracion = match parsear_argumentos() {
        Some(cfg) => cfg,
        None => std::process::exit(CODIGO_ERROR),
    };

    let conteo_global = procesar_archivo_con_pool(
        configuracion.ruta_archivo.clone(),
        configuracion.cantidad_threads,
    );

    let salida_final = preparar_salida_final(&conteo_global);
    guardar_json_de_salida(&salida_final, &configuracion.nombre_archivo_salida);
}
