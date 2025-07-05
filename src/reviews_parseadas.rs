//! Este módulo contiene la estructura usada para el parseo incial del '.csv'.

// Imports de crates externas.
use serde::Deserialize;

/// Estructura que representa una reseña individual de un archivo `.csv`.
///
/// Esta estructura se usa para deserializar directamente los campos del CSV,
/// renombrando los encabezados originales a nombres más descriptivos en español.
#[derive(Debug, Deserialize, Clone)]
pub struct Reseña {
    /// Nombre del juego (`app_name` en el CSV original).
    #[serde(rename = "app_name")]
    pub nombre_juego: String,

    /// Idioma en el que está escrita la reseña (`language`).
    #[serde(rename = "language")]
    pub idioma: String,

    /// Texto completo de la reseña (`review`).
    #[serde(rename = "review")]
    pub texto: String,

    /// Cantidad de votos útiles recibidos (`votes_helpful`).
    ///
    /// Se mantiene como `String` al parsearse y se convierte a `u32` más adelante.
    #[serde(rename = "votes_helpful")]
    pub votos_utiles: String,
}
