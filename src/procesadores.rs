//! Este módulo contiene la lógica de los hilos del programa que procesan la informacion.

// Imports de crates externas.
use csv::ReaderBuilder;
use rayon::prelude::*;
use std::error::Error;
use std::fs::{File, read_dir};
use std::path::Path;
use std::sync::mpsc::{self, Sender};

// Imports de funciones/estructuras propias.
use crate::estadisticas::EstadisticasParciales;
use crate::reviews_parseadas::Reseña;

// Constantes.
const CHUNK_SIZE: usize = 100_000;
const EXTENSION_ARCHIVO_A_PROCESAR: &str = "csv";

// Mensajes.
const ERROR_PATH_INVALIDO: &str = "❌ El path no es un directorio válido.";
const ERROR_ABRIR_ARCHIVO: &str = "⚠️ Error al abrir el archivo";
const ERROR_ENTRADA_DIRECTORIO: &str = "⚠️ Error al leer entrada del directorio:";

/// Valida que el path recibido sea un directorio válido.
///
/// # Argumentos
/// * `path` - Ruta del directorio a validar.
///
/// # Retorna
/// * `Ok(())` si es válido, o un error si no lo es.
fn validar_directorio(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() || !path.is_dir() {
        return Err(ERROR_PATH_INVALIDO.into());
    }

    Ok(())
}

/// Procesa un archivo `.csv` leyendo sus reseñas y agrupándolas en chunks.
///
/// # Argumentos
/// * `ruta` - Ruta del archivo a procesar.
/// * `tx` - Canal para enviar los chunks hacia los hilos trabajadores.
fn procesar_archivo_csv(ruta: &Path, tx: Sender<Vec<Reseña>>) {
    let archivo = match File::open(ruta) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} {}: {}", ERROR_ABRIR_ARCHIVO, ruta.display(), e);
            return;
        }
    };

    let mut lector = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(archivo);

    let mut buffer_chunk = Vec::with_capacity(CHUNK_SIZE);
    for reseña in lector.deserialize::<Reseña>().flatten() {
        buffer_chunk.push(reseña);
        if buffer_chunk.len() == CHUNK_SIZE {
            tx.send(std::mem::take(&mut buffer_chunk)).unwrap();
        }
    }

    if !buffer_chunk.is_empty() {
        tx.send(buffer_chunk).unwrap();
    }
}

/// Lanza un hilo 'Productor' que recorre el directorio buscando archivos `.csv`,
/// y manda los chunks al canal correspondiente.
///
/// # Argumentos
/// * `dir_path` - Ruta del directorio a escanear.
/// * `tx` - Canal para enviar los chunks.
///
/// # Retorna
/// * `JoinHandle<()>` del hilo productor.
fn spawn_productor_directorio(
    dir_path: &Path,
    tx: Sender<Vec<Reseña>>,
) -> std::thread::JoinHandle<()> {
    let dir = dir_path.to_path_buf();
    std::thread::spawn(move || {
        for entry in read_dir(&dir).unwrap() {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("{} {}", ERROR_ENTRADA_DIRECTORIO, e);
                    continue;
                }
            };

            let ruta = entry.path();
            if ruta.is_file()
                && ruta
                    .extension()
                    .is_some_and(|ext| ext == EXTENSION_ARCHIVO_A_PROCESAR)
            {
                procesar_archivo_csv(&ruta, tx.clone());
            }
        }
    })
}

/// Función principal que coordina el procesamiento de todos los `.csv` de un directorio.
///
/// Lanza el hilo productor y luego usa la pool de Rayon para procesar los chunks de reseñas en paralelo.
///
/// # Argumentos
/// * `directorio` - Ruta del directorio a procesar.
///
/// # Retorna
/// * `Ok(Vec<EstadisticasParciales>)` con las estadísticas generadas o un error si falló algo.
pub fn procesar_csv_con_rayon(
    directorio: String,
) -> Result<Vec<EstadisticasParciales>, Box<dyn Error>> {
    let path = Path::new(&directorio);
    validar_directorio(path)?;
    let (tx, rx) = mpsc::channel::<Vec<Reseña>>();
    let productor = spawn_productor_directorio(path, tx);
    let parciales: Vec<EstadisticasParciales> =
        rx.into_iter().par_bridge().map(procesar_chunk).collect();

    productor.join().unwrap();
    Ok(parciales)
}

/// Procesa un chunk de reseñas y genera las estadísticas parciales correspondientes.
///
/// # Argumentos
/// * `chunk` - Vector de reseñas a analizar.
///
/// # Retorna
/// * `EstadisticasParciales` con la información procesada del chunk.
fn procesar_chunk(chunk: Vec<Reseña>) -> EstadisticasParciales {
    let mut stats = EstadisticasParciales::default();
    for reseña in chunk {
        let juego = reseña.nombre_juego;
        let idioma = reseña.idioma;
        let texto = reseña.texto;
        let votos_resultado = reseña.votos_utiles.parse::<u32>();
        if let Ok(votos) = votos_resultado {
            let entry = stats.juegos.entry(juego).or_default();
            entry.cantidad_total += 1;
            *entry.por_idioma.entry(idioma.clone()).or_insert(0) += 1;
            match entry.mejores_reviews.entry(idioma.clone()) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    if votos > e.get().1 {
                        e.insert((texto.clone(), votos));
                    }
                }

                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert((texto.clone(), votos));
                }
            }

            let idioma_entry = stats.por_idioma.entry(idioma).or_default();
            idioma_entry.cantidad_total += 1;
            idioma_entry.top_reviews.push((texto, votos));
        }
    }

    stats
}
